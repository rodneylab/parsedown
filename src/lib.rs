mod html_process;
mod markdown;

use pulldown_cmark::{html, Event, Options, Parser, Tag};
use std::io::Cursor;
use wasm_bindgen::prelude::*;

use html_process::process_html;
use markdown::slugified_title;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn markdown_to_html(markdown: &str) -> String {
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

    html::write_html(Cursor::new(&mut bytes), heading_parser).unwrap();

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
        Ok(_) => {
            let parse_html = String::from_utf8_lossy(&bytes).to_string();
            process_html(&parse_html)
        }
        Err(error) => {
            console_log!("Error parsing markdown: {error}");
            String::from("")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_markdown_to_html() {
        let markdown = r#"
hello
=====

* alpha
* beta
"#;

        let result = markdown_to_html(markdown);
        assert_eq!(
            result,
            r#"<h1>hello</h1>
<ul>
<li>alpha</li>
<li>beta</li>
</ul>
"#
        );
    }
}
