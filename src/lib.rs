#![warn(clippy::all, clippy::pedantic)]

mod html_process;
mod inline_html;
mod markdown;
mod url_utility;
mod utilities;

use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::*, JsValue};

use crate::{
    html_process::process_html,
    markdown::{
        parse_markdown_to_html, parse_markdown_to_plaintext, Heading, ParseMarkdownOptions,
        TextStatistics,
    },
};

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

#[derive(Deserialize)]
pub struct ParseInputOptions {
    canonical_root_url: Option<String>,
    enable_smart_punctuation: Option<bool>,
    search_term: Option<String>,
}

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct ParseResults {
    html: Option<String>,
    headings: Option<Vec<Heading>>,
    statistics: Option<TextStatistics>,
    errors: Option<Vec<String>>,
}

fn markdown_to_processed_html(markdown: &str, options: &ParseInputOptions) -> ParseResults {
    match parse_markdown_to_html(markdown) {
        Ok((html_value, headings, statistics_value)) => {
            let html = Some(process_html(
                &html_value,
                options.canonical_root_url.as_deref(),
                options.search_term.as_deref(),
            ));
            let headings = Some(headings);
            let statistics = Some(statistics_value);
            ParseResults {
                html,
                headings,
                statistics,
                errors: None,
            }
        }
        Err(error) => {
            let message = format!("Error parsing markdown: {error}");
            let errors = vec![message];
            ParseResults {
                html: None,
                headings: None,
                statistics: None,
                errors: Some(errors),
            }
        }
    }
}

/// # Panics
///
/// Will panic if unable to parse options
#[wasm_bindgen]
#[must_use]
pub fn markdown_to_html(markdown: &str, options: JsValue) -> JsValue {
    let input_options: Option<ParseInputOptions> = serde_wasm_bindgen::from_value(options).unwrap();
    let parse_options = match input_options {
        Some(value) => value,
        None => ParseInputOptions {
            canonical_root_url: None,
            enable_smart_punctuation: Some(true),
            search_term: None,
        },
    };
    let results = markdown_to_processed_html(markdown, &parse_options);
    serde_wasm_bindgen::to_value(&results).unwrap()
}

/// # Panics
///
/// Will panic if unable to parse options
#[wasm_bindgen]
#[must_use]
pub fn markdown_to_plaintext(markdown: &str, options: JsValue) -> String {
    let input_options: Option<ParseInputOptions> = serde_wasm_bindgen::from_value(options).unwrap();
    let mut markdown_options = ParseMarkdownOptions::default();

    if let Some(ParseInputOptions {
        canonical_root_url,
        enable_smart_punctuation,
        ..
    }) = input_options
    {
        let canonical_root_url = canonical_root_url.as_deref();
        markdown_options.canonical_root_url(canonical_root_url);

        if let Some(value) = enable_smart_punctuation {
            markdown_options.enable_smart_punctuation(value);
        }
        parse_markdown_to_plaintext(markdown, &markdown_options)
    } else {
        parse_markdown_to_plaintext(markdown, &markdown_options)
    }
}

#[wasm_bindgen]
#[must_use]
pub fn mjml_to_html(mjml: &str) -> String {
    let root = match mrml::parse(mjml) {
        Ok(value) => value,
        Err(error) => {
            console_log!("Error parsing mjml: {:?}", error);
            return String::new();
        }
    };
    let opts = mrml::prelude::render::RenderOptions::default();
    match root.element.render(&opts) {
        Ok(value) => value,
        Err(error) => {
            console_log!("Error rendering parsed mjml to html: {:?}", error);
            String::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_markdown_to_html() {
        let markdown = r"
hello you
=========

* alpha
* beta
";

        let result = markdown_to_processed_html(
            markdown,
            &ParseInputOptions {
                canonical_root_url: None,
                enable_smart_punctuation: Some(true),
                search_term: None,
            },
        );
        let html = Some(String::from(
            r#"<h1 id="hello-you">hello you</h1>
<ul>
<li>alpha</li>
<li>beta</li>
</ul>
"#,
        ));
        assert_eq!(
            result,
            ParseResults {
                html,
                headings: Some(vec![Heading::new("hello you", "hello-you")]),
                statistics: Some(TextStatistics::new(4)),
                errors: None
            }
        );

        let markdown = "
## Subheading

Paragraph text.
";

        let result = markdown_to_processed_html(
            markdown,
            &ParseInputOptions {
                canonical_root_url: None,
                enable_smart_punctuation: Some(true),
                search_term: None,
            },
        );
        let html = Some(String::from(
            r##"<h2 id="subheading">Subheading <a href="#subheading" class="heading-anchor">#</a></h2>
<p>Paragraph text.</p>
"##,
        ));
        assert_eq!(
            result,
            ParseResults {
                html,
                headings: Some(vec![Heading::new("Subheading", "subheading")]),
                statistics: Some(TextStatistics::new(3)),
                errors: None
            },
        );

        let markdown = "
### Subheading

Link: [Example site](https://example.com).
";

        let result = markdown_to_processed_html(
            markdown,
            &ParseInputOptions {
                canonical_root_url: None,
                enable_smart_punctuation: Some(true),
                search_term: None,
            },
        );
        let html = Some(String::from(
            r#"<h3 id="subheading">Subheading</h3>
<p>Link: <a href="https://example.com" target="_blank" rel="nofollow noopener noreferrer">Example site</a>.</p>
"#,
        ));
        assert_eq!(
            result,
            ParseResults {
                html,
                headings: Some(vec![Heading::new("Subheading", "subheading")]),
                statistics: Some(TextStatistics::new(4)),
                errors: None
            }
        );
    }

    #[test]
    fn test_mjml_to_html() {
        let mjml = r"<mjml></mjml>";
        let result = mjml_to_html(mjml);
        let expected = r#"<!doctype html><html xmlns="http://www.w3.org/1999/xhtml" xmlns:v="urn:schemas-microsoft-com:vml" xmlns:o="urn:schemas-microsoft-com:office:office"><head><title></title><!--[if !mso]><!--><meta http-equiv="X-UA-Compatible" content="IE=edge"><!--<![endif]--><meta http-equiv="Content-Type" content="text/html; charset=UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1">
<style type="text/css">
#outlook a { padding: 0; }
body { margin: 0; padding: 0; -webkit-text-size-adjust: 100%; -ms-text-size-adjust: 100%; }
table, td { border-collapse: collapse; mso-table-lspace: 0pt; mso-table-rspace: 0pt; }
img { border: 0; height: auto; line-height: 100%; outline: none; text-decoration: none; -ms-interpolation-mode: bicubic; }
p { display: block; margin: 13px 0; }
</style>
<!--[if mso]>
<noscript>
<xml>
<o:OfficeDocumentSettings>
  <o:AllowPNG/>
  <o:PixelsPerInch>96</o:PixelsPerInch>
</o:OfficeDocumentSettings>
</xml>
</noscript>
<![endif]-->
<!--[if lte mso 11]>
<style type="text/css">
.mj-outlook-group-fix { width:100% !important; }
</style>
<![endif]-->
<style type="text/css"></style></head><body></body></html>"#;
        assert_eq!(result, expected);
    }
}
