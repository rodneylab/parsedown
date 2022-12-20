#[cfg(test)]
mod tests;

use deunicode::deunicode;
use pulldown_cmark::{html, Event, Options, Parser, Tag};
use std::io::{Cursor, Error};

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

pub fn parse_markdown_to_html(markdown: &str) -> Result<String, Error> {
    let mut bytes = Vec::new();
    let mut options = Options::empty();
    options.insert(Options::ENABLE_SMART_PUNCTUATION);

    let mut heading_identifiers: Vec<String> = Vec::new();
    let mut current_id_fragments = String::new();
    let mut parsing_heading = false;

    let heading_parser = Parser::new_ext(markdown, options).map(|event| {
        match &event {
            Event::Start(Tag::Heading(_level, _identifier, _classes)) => {
                parsing_heading = true;
            }
            Event::Code(value) | Event::Text(value) => {
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
        Ok(_) => Ok(String::from_utf8_lossy(&bytes).to_string()),
        Err(error) => Err(error),
    }
}
