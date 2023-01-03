#[cfg(test)]
mod tests;

use crate::url_utility::relative_url;

use deunicode::deunicode;
use nom::{
    self,
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::multispace0,
    sequence::{delimited, pair},
    IResult,
};
use pulldown_cmark::{
    escape::StrWrite,
    html,
    Event::{self, Code, End, Html, SoftBreak, Start, Text},
    Options, Parser, Tag,
};
use serde::Serialize;
use std::io::{self, Cursor};
use textwrap::wrap;

/// Reading time in minutes from number of words, assumes 180 wpm reading speed from a device
fn reading_time_from_words(words: u32) -> u32 {
    let result = (words as f64 / 180.0).round();
    if result > 0.0 {
        result as u32
    } else {
        1
    }
}

/// Emoji are not included in word count and hyphenated, compound words (half-time) are one word
fn words(text: &str) -> u32 {
    text.split(|c| char::is_whitespace(c) || c == '/')
        .fold(0, |acc, x| {
            // only count as a word if there is at least one alphanumeric character or is &
            if x.contains(char::is_alphanumeric) || x == "&" {
                acc + 1
            } else {
                acc
            }
        })
}

fn slugified_title(title: &str) -> String {
    let deunicoded_title = deunicode(title);
    let mut result = String::with_capacity(deunicoded_title.len());
    let mut last_was_replaced = true;
    let remove_characters = "?'`:[]()!";
    let replace_characters = " -/.,";
    for chars in deunicoded_title.chars() {
        if replace_characters.contains(chars) {
            if !last_was_replaced {
                last_was_replaced = true;
                result.push('-');
            }
        } else if !remove_characters.contains(chars) {
            last_was_replaced = false;
            result.push_str(&chars.to_lowercase().to_string());
        }
    }
    result
}

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct TextStatistics {
    reading_time: u32,
    word_count: u32,
}

impl TextStatistics {
    #[allow(dead_code)]
    pub fn new(word_count: u32) -> TextStatistics {
        let reading_time = reading_time_from_words(word_count);
        TextStatistics {
            reading_time,
            word_count,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct Heading {
    heading: String,
    id: String,
}

impl Heading {
    pub fn new(heading: &str, id: &str) -> Heading {
        Heading {
            heading: heading.into(),
            id: id.into(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

pub fn parse_markdown_to_html(
    markdown: &str,
) -> io::Result<(String, Vec<Heading>, TextStatistics)> {
    let mut bytes = Vec::new();
    let mut options = Options::empty();
    options.insert(Options::ENABLE_SMART_PUNCTUATION);

    let mut headings: Vec<Heading> = Vec::new();
    let mut current_id_fragments = String::new();
    let mut parsing_heading = false;
    let mut word_count: u32 = 0;

    let heading_parser = Parser::new_ext(markdown, options).map(|event| {
        match &event {
            Event::Start(Tag::Heading(_level, _identifier, _classes)) => {
                parsing_heading = true;
            }
            Event::Text(value) => {
                word_count += words(value);
                if parsing_heading {
                    current_id_fragments.push_str(value);
                }
            }
            Event::Code(value) => {
                if parsing_heading {
                    current_id_fragments.push_str(value);
                }
            }
            Event::End(Tag::Heading(_level, _identifier, _classes)) => {
                let heading = &current_id_fragments;
                let id = slugified_title(&current_id_fragments);
                headings.push(Heading::new(heading, &id));
                current_id_fragments = String::new();
                parsing_heading = false;
            }
            _ => {}
        }
        event
    });
    html::write_html(Cursor::new(&mut bytes), heading_parser)?;
    let reading_time = reading_time_from_words(word_count);
    let statistics = TextStatistics {
        reading_time,
        word_count,
    };

    let mut heading_iterator = headings.iter();
    let parser = Parser::new_ext(markdown, options).map(|event| match &event {
        Event::Start(Tag::Heading(level, _identifiers, _classes)) => {
            let heading_identifier = heading_iterator.next();
            Event::Start(Tag::Heading(
                *level,
                heading_identifier.map(|x| x.id()),
                Vec::new(),
            ))
        }
        _ => event,
    });

    match html::write_html(Cursor::new(&mut bytes), parser) {
        Ok(_) => Ok((
            String::from_utf8_lossy(&bytes).to_string(),
            headings,
            statistics,
        )),
        Err(error) => Err(error),
    }
}

enum HTMLElementEvent {
    Start,
    End,
}

fn is_html_tag_name_start(character: char) -> bool {
    character.is_alphabetic() || character == '_'
}
fn is_html_tag_name(character: char) -> bool {
    is_html_tag_name_start(character) || character.is_ascii_digit() || "-.".contains(character)
}

fn parse_html_tag_content(line: &str) -> IResult<&str, &str> {
    let (rest, tag_content) = is_not(">/")(line)?;
    let (_attributes, (tag_name, _space)) = pair(is_not(" "), multispace0)(tag_content)?;
    Ok((rest, tag_name))
}

fn parse_closing_html_tag(line: &str) -> IResult<&str, (HTMLElementEvent, &str)> {
    let (rest, tag_name) = delimited(tag("</"), parse_html_tag_content, tag(">"))(line)?;
    Ok((rest, (HTMLElementEvent::End, tag_name)))
}

fn parse_opening_html_tag(line: &str) -> IResult<&str, (HTMLElementEvent, &str)> {
    let (rest, tag_name) = delimited(tag("<"), parse_html_tag_content, tag(">"))(line)?;
    Ok((rest, (HTMLElementEvent::Start, tag_name)))
}

fn html_tag(html: &str) -> Result<(HTMLElementEvent, &str), Box<dyn std::error::Error>> {
    match alt((parse_opening_html_tag, parse_closing_html_tag))(html) {
        Ok((_rest, (event, tag_name))) => {
            // check the HTML tag name is valid
            /* todo(rodneylab): check if it is possible to receive an invalid closing or opening
             * tag name
             */
            let mut character_iter = tag_name.chars();
            if let Some(value) = character_iter.next() {
                if !is_html_tag_name_start(value) {
                    return Err(format!("Invalid HTML element name: {tag_name}").into());
                }
            } else {
                // todo(rodneylab): check if this can happen - might be able to remove arm
                return Err(format!("Invalid HTML element name: {tag_name}").into());
            }
            for character in character_iter {
                if !is_html_tag_name(character) {
                    return Err(format!("Invalid HTML element name: {tag_name}").into());
                }
            }
            Ok((event, tag_name))
        }
        Err(error) => Err(format!("{:?}", error).into()),
    }
}

struct PlaintextWriter<'a, I, W> {
    /// Iterator supplying events.
    iter: I,

    /// Writer to write to.
    writer: W,

    /// Whether the last write created a newline
    end_newline: bool,

    /// Buffer of words in current line of input, gets wrapped to preferred length before output
    current_line: String,

    /// Preferred length of wrapped out lines, currently fixed at 72
    line_length: usize,

    /// HTML tags to ignore in ouput
    ignore_tags: Vec<&'a str>,

    /// Optionally prepended to relative URLs
    canonical_root_url: Option<&'a str>,
}

impl<'a, I, W> PlaintextWriter<'a, I, W>
where
    I: Iterator<Item = Event<'a>>,
    W: StrWrite,
{
    fn new(iter: I, writer: W, canonical_root_url: Option<&'a str>) -> Self {
        Self {
            iter,
            writer,
            end_newline: true,
            current_line: String::new(),
            line_length: 72,
            ignore_tags: vec!["tool-tip"],
            canonical_root_url,
        }
    }

    /// Writes a new line.
    fn write_newline(&mut self) -> io::Result<()> {
        self.end_newline = true;
        self.writer.write_str("\n")
    }

    /// Wraps the current line on input to preferred length and writes the wrapped lines
    #[inline]
    fn write(&mut self) -> io::Result<()> {
        let lines = wrap(&self.current_line, self.line_length);
        for line in lines.iter() {
            self.writer.write_str(line)?;
            self.writer.write_str("\n")?;
        }

        self.current_line = String::new();
        if !self.current_line.is_empty() {
            self.end_newline = self.current_line.ends_with('\n');
        }
        Ok(())
    }

    fn run(mut self) -> io::Result<()> {
        while let Some(event) = self.iter.next() {
            match event {
                Start(tag) => {
                    self.start_tag(tag)?;
                }
                End(tag) => {
                    self.end_tag(tag)?;
                }
                Text(text) => {
                    self.current_line.push_str(&text);
                    self.end_newline = text.ends_with('\n');
                }
                Code(text) => {
                    self.current_line.push_str(&text);
                    self.end_newline = text.ends_with('\n');
                }
                Html(html) => match html_tag(&html) {
                    Ok((HTMLElementEvent::Start, tag_name_value)) => {
                        if self.ignore_tags.contains(&tag_name_value) {
                            for html_event in self.iter.by_ref() {
                                if let Html(html_value) = html_event {
                                    if let Ok((HTMLElementEvent::End, end_tag_name_value)) =
                                        html_tag(&html_value)
                                    {
                                        if end_tag_name_value == tag_name_value {
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Ok((_event, _tag_name)) => {}
                    Err(_) => {}
                },
                SoftBreak => {
                    self.current_line.push(' ');
                }
                _ => {}
            }
        }
        Ok(())
    }

    /// Handles the start of an HTML tag.
    fn start_tag(&mut self, tag: Tag) -> io::Result<()> {
        match tag {
            Tag::Paragraph => {
                if !self.end_newline {
                    self.write()
                } else {
                    Ok(())
                }
            }
            Tag::Heading(_level, _id, _classes) => {
                if self.end_newline {
                    self.end_newline = false;
                    Ok(())
                } else {
                    self.write()?;
                    self.write_newline()
                }
            }
            Tag::Item => {
                if self.end_newline {
                    self.current_line.push_str("- ");
                } else {
                    self.current_line.push_str("\n- ");
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn end_tag(&mut self, tag: Tag) -> io::Result<()> {
        match tag {
            Tag::Paragraph => {
                self.write()?;
            }
            Tag::Heading(_level, _id, _classes) => {
                self.write()?;
            }
            Tag::Item => {
                self.write()?;
            }
            Tag::Link(_link_type, dest, _title) => {
                self.current_line.push_str(" (");
                if let Some(root_url_value) = self.canonical_root_url {
                    if relative_url(&dest) {
                        self.current_line.push_str(root_url_value);
                    }
                }
                self.current_line.push_str(&dest);

                self.current_line.push(')');
            }
            _ => {}
        }
        Ok(())
    }
}

fn push_plaintext<'a, I>(s: &mut String, iter: I, canonical_root_url: Option<&'a str>)
where
    I: Iterator<Item = Event<'a>>,
{
    PlaintextWriter::new(iter, s, canonical_root_url)
        .run()
        .unwrap();
}

pub fn parse_markdown_to_plaintext(markdown: &str, canonical_root_url: Option<&str>) -> String {
    let mut plaintext_buf = String::new();
    let mut options = Options::empty();
    options.insert(Options::ENABLE_SMART_PUNCTUATION);
    let parser = Parser::new_ext(markdown, Options::empty());
    push_plaintext(&mut plaintext_buf, parser, canonical_root_url);
    plaintext_buf
}
