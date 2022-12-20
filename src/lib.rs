mod html_process;
mod markdown;

use html_process::process_html;
use markdown::parse_markdown_to_html;
use wasm_bindgen::prelude::*;

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
    match parse_markdown_to_html(markdown) {
        Ok(value) => process_html(&value),
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
            r#"<h1 id="hello">hello</h1>
<ul>
<li>alpha</li>
<li>beta</li>
</ul>
"#
        );
    }
}
