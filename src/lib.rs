mod html_process;
mod markdown;

use html_process::process_html;
use markdown::{parse_markdown_to_html, TextStatistics};
use serde::Serialize;
use wasm_bindgen::{prelude::*, JsValue};

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

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct ParseResults {
    html: Option<String>,
    statistics: Option<TextStatistics>,
    errors: Option<Vec<String>>,
}

fn markdown_to_processed_html(markdown: &str) -> ParseResults {
    match parse_markdown_to_html(markdown) {
        Ok((html_value, statistics_value)) => {
            let html = Some(process_html(&html_value));
            let statistics = Some(statistics_value);
            ParseResults {
                html,
                statistics,
                errors: None,
            }
        }
        Err(error) => {
            let message = format!("Error parsing markdown: {error}");
            let errors = vec![message];
            ParseResults {
                html: None,
                statistics: None,
                errors: Some(errors),
            }
        }
    }
}

#[wasm_bindgen]
pub fn markdown_to_html(markdown: &str) -> JsValue {
    let results = markdown_to_processed_html(markdown);
    serde_wasm_bindgen::to_value(&results).unwrap()
}

#[wasm_bindgen]
pub fn mjml_to_html(mjml: &str) -> String {
    let root = match mrml::parse(mjml) {
        Ok(value) => value,
        Err(error) => {
            console_log!("Error parsing mjml: {:?}", error);
            return String::from("");
        }
    };
    let opts = mrml::prelude::render::Options::default();
    match root.render(&opts) {
        Ok(value) => value,
        Err(error) => {
            console_log!("Error rendering parsed mjml to html: {:?}", error);
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

        let result = markdown_to_processed_html(markdown);
        let html = Some(String::from(
            r#"<h1 id="hello">hello</h1>
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
                statistics: Some(TextStatistics::new(3)),
                errors: None
            }
        );

        let markdown = r#"
## Subheading

Paragraph text.
"#;

        let result = markdown_to_processed_html(markdown);
        let html = Some(String::from(
            r##"<h2 id="subheading">Subheading <a href="#subheading" class="heading-anchor">#</a></h2>
<p>Paragraph text.</p>
"##,
        ));
        assert_eq!(
            result,
            ParseResults {
                html,
                statistics: Some(TextStatistics::new(3)),
                errors: None
            }
        );

        let markdown = r#"
### Subheading

Link: [Example site](https://example.com).
"#;

        let result = markdown_to_processed_html(markdown);
        let html = Some(String::from(
            r##"<h3 id="subheading">Subheading</h3>
<p>Link: <a href="https://example.com" rel="nofollow noopener noreferrer">Example site</a>.</p>
"##,
        ));
        assert_eq!(
            result,
            ParseResults {
                html,
                statistics: Some(TextStatistics::new(4)),
                errors: None
            }
        );
    }

    #[test]
    fn test_mjml_to_html() {
        let mjml = r#"<mjml></mjml>"#;
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
</head><body></body></html>"#;
        assert_eq!(result, expected);
    }
}
