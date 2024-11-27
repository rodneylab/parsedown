#[cfg(test)]
mod tests;

use std::io::{self, Cursor};

use deunicode::deunicode;
use pulldown_cmark::{
    html, CowStr,
    Event::{self, Code, End, InlineHtml, SoftBreak, Start, Text},
    Options, Parser, Tag, TagEnd,
};
use pulldown_cmark_escape::StrWrite;
use serde::Serialize;
use textwrap::wrap;

use crate::{
    inline_html::{parse_node as parse_inline_html_node, InlineHTMLTagType},
    url_utility::relative_url,
    utilities::stack::Stack,
};

/// Reading time in minutes from number of words, assumes 180 wpm reading speed from a device
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn reading_time_from_words(words: u32) -> u32 {
    let result = (f64::from(words) / 180.0).round();
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

    let heading_parser = Parser::new_ext(markdown, options).inspect(|event| match &event {
        Event::Start(Tag::Heading { .. }) => {
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
        Event::End(TagEnd::Heading(_heading_level)) => {
            let heading = &current_id_fragments;
            let id = slugified_title(&current_id_fragments);
            headings.push(Heading::new(heading, &id));
            current_id_fragments = String::new();
            parsing_heading = false;
        }
        _ => {}
    });
    html::write_html_io(Cursor::new(&mut bytes), heading_parser)?;
    let reading_time = reading_time_from_words(word_count);
    let statistics = TextStatistics {
        reading_time,
        word_count,
    };

    let mut heading_iterator = headings.iter();
    let parser = Parser::new_ext(markdown, options).map(|event| match &event {
        Event::Start(Tag::Heading { level, .. }) => {
            let heading_identifier = heading_iterator.next();
            Event::Start(Tag::Heading {
                level: *level,
                id: heading_identifier.map(|val| CowStr::from(val.id())),
                classes: Vec::new(),
                attrs: Vec::new(),
            })
        }
        _ => event,
    });

    match html::write_html_io(Cursor::new(&mut bytes), parser) {
        Ok(()) => Ok((
            String::from_utf8_lossy(&bytes).to_string(),
            headings,
            statistics,
        )),
        Err(error) => Err(error),
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

    current_link: Option<String>,

    /// Preferred length of wrapped out lines, currently fixed at 72
    line_length: usize,

    /// HTML tags to ignore in output
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
            current_link: None,
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
        for line in &lines {
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
                Text(text) | Code(text) => {
                    self.current_line.push_str(&text);
                    self.end_newline = text.ends_with('\n');
                }
                InlineHtml(inline_html) => {
                    if let Some(InlineHTMLTagType::Opening(value)) =
                        parse_inline_html_node(&inline_html)
                    {
                        if self.ignore_tags.contains(&value.as_ref()) {
                            let mut open_tags: Stack<String> = Stack::new();
                            open_tags.push(value);
                            for html_event in self.iter.by_ref() {
                                if let InlineHtml(nested_inline_html) = html_event {
                                    match parse_inline_html_node(&nested_inline_html) {
                                        Some(InlineHTMLTagType::Opening(open_tag_value)) => {
                                            open_tags.push(open_tag_value);
                                        }
                                        Some(InlineHTMLTagType::Closing(closing_tag_value)) => {
                                            if let Some(popped_value) = open_tags.pop() {
                                                if popped_value == closing_tag_value
                                                    && open_tags.is_empty()
                                                {
                                                    break;
                                                }
                                            }
                                        }
                                        None => {}
                                    }
                                }
                            }
                        }
                    }
                }
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
                if self.end_newline {
                    Ok(())
                } else {
                    self.write()
                }
            }
            Tag::Heading { .. } => {
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
            Tag::Link { dest_url, .. } => {
                self.current_link = Some(dest_url.to_string());
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn end_tag(&mut self, tag: TagEnd) -> io::Result<()> {
        match tag {
            TagEnd::Paragraph => {
                self.write()?;
            }
            TagEnd::Heading(_level) => {
                self.write()?;
            }
            TagEnd::Item => {
                self.write()?;
            }
            TagEnd::Link => {
                if let Some(value) = &self.current_link {
                    self.current_line.push_str(" (");
                    if let Some(root_url_value) = self.canonical_root_url {
                        if relative_url(value) {
                            self.current_line.push_str(root_url_value);
                        }
                    }
                    self.current_line.push_str(value);
                    self.current_line.push(')');
                }
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

#[derive(Debug)]
pub struct ParseMarkdownOptions<'a> {
    canonical_root_url: Option<&'a str>,
    enable_smart_punctuation: bool,
}

impl<'a> Default for ParseMarkdownOptions<'a> {
    fn default() -> Self {
        ParseMarkdownOptions {
            canonical_root_url: None,
            enable_smart_punctuation: true,
        }
    }
}

impl<'a> ParseMarkdownOptions<'a> {
    pub fn canonical_root_url(&mut self, value: Option<&'a str>) -> &mut Self {
        self.canonical_root_url = value;
        self
    }

    pub fn enable_smart_punctuation(&mut self, value: bool) -> &mut Self {
        self.enable_smart_punctuation = value;
        self
    }
}

pub fn parse_markdown_to_plaintext(markdown: &str, options: &ParseMarkdownOptions) -> String {
    let ParseMarkdownOptions {
        canonical_root_url,
        enable_smart_punctuation,
    } = options;

    let mut parser_options = Options::empty();
    if *enable_smart_punctuation {
        parser_options.insert(Options::ENABLE_SMART_PUNCTUATION);
    }
    let parser = Parser::new_ext(markdown, parser_options);

    let mut plaintext_buf = String::new();
    push_plaintext(&mut plaintext_buf, parser, *canonical_root_url);
    plaintext_buf
}
