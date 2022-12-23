#[cfg(test)]
mod tests;

use deunicode::deunicode;
use pulldown_cmark::{html, Event, Options, Parser, Tag};
use serde::Serialize;
use std::io::{Cursor, Error};

/// Emoji are not included in word count and hyphenated, compound words (half-time) are one word
fn words(text: &str) -> u64 {
    text.split(|c| char::is_whitespace(c) || c == '/')
        .fold(0, |acc, x| {
            // only count as a word if there is at least one alphanumeric character or is &
            if x.find(char::is_alphanumeric).is_some() || x == "&" {
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
    word_count: u64,
}

impl TextStatistics {
    #[allow(dead_code)]
    pub fn new(word_count: u64) -> TextStatistics {
        TextStatistics { word_count }
    }
}

pub fn parse_markdown_to_html(markdown: &str) -> Result<(String, TextStatistics), Error> {
    let mut bytes = Vec::new();
    let mut options = Options::empty();
    options.insert(Options::ENABLE_SMART_PUNCTUATION);

    let mut heading_identifiers: Vec<String> = Vec::new();
    let mut current_id_fragments = String::new();
    let mut parsing_heading = false;
    let mut word_count: u64 = 0;

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
                let slugified_title_value = slugified_title(&current_id_fragments);
                heading_identifiers.push(slugified_title_value);
                current_id_fragments = String::new();
                parsing_heading = false;
            }
            _ => {}
        }
        event
    });
    html::write_html(Cursor::new(&mut bytes), heading_parser)?;
    let statistics = TextStatistics { word_count };

    let mut heading_identifiers_iterator = heading_identifiers.iter();
    let parser = Parser::new_ext(markdown, options).map(|event| match &event {
        Event::Start(Tag::Heading(level, _identifiers, _classes)) => {
            let heading_identifier = heading_identifiers_iterator.next();
            Event::Start(Tag::Heading(
                *level,
                heading_identifier.map(|x| &**x),
                Vec::new(),
            ))
        }
        _ => event,
    });

    match html::write_html(Cursor::new(&mut bytes), parser) {
        Ok(_) => Ok((String::from_utf8_lossy(&bytes).to_string(), statistics)),
        Err(error) => Err(error),
    }
}
