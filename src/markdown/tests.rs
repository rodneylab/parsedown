use crate::markdown::{
    parse_markdown_to_html, parse_markdown_to_plaintext, reading_time_from_words, slugified_title,
    words, ParseMarkdownOptions,
};

#[test]
fn test_reading_time_from_words() {
    assert_eq!(reading_time_from_words(179), 1);
    assert_eq!(reading_time_from_words(0), 1);
    assert_eq!(reading_time_from_words(180), 1);
    assert_eq!(reading_time_from_words(181), 1);
    assert_eq!(reading_time_from_words(269), 1);
    assert_eq!(reading_time_from_words(270), 2);
}

#[test]
fn test_words() {
    let text = "hello";
    assert_eq!(words(text), 1);

    let text = "half-time";
    assert_eq!(words(text), 1);

    let text = "";
    assert_eq!(words(text), 0);

    let text = "A complete sentence. ";
    assert_eq!(words(text), 3);

    let text = "Resources - writing";
    assert_eq!(words(text), 2);

    let text = "He/she/they";
    assert_eq!(words(text), 3);

    let text = "Acme & co.";
    assert_eq!(words(text), 3);
}

#[test]
fn test_parse_markdown_to_html() {
    let markdown = r#"
hello
=====

* alpha
* beta
"#;

    let result =
        if let Some((result, _headings, _statistics)) = parse_markdown_to_html(markdown).ok() {
            result
        } else {
            panic!("Result expected");
        };
    let expected = String::from(
        r#"<h1 id="hello">hello</h1>
<ul>
<li>alpha</li>
<li>beta</li>
</ul>
"#,
    );
    assert_eq!(result, expected);
}

#[test]
fn test_parse_markdown_to_plaintext() {
    let markdown = "## 🧑🏽‍🍳 Pick of the Month — vanilla-extract";

    let result = parse_markdown_to_plaintext(markdown, ParseMarkdownOptions::default());
    let expected = String::from("🧑🏽\u{200d}🍳 Pick of the Month — vanilla-extract\n");
    assert_eq!(result, expected);

    let markdown = "My apple's quite tasty.";
    let result = parse_markdown_to_plaintext(markdown, ParseMarkdownOptions::default());
    let expected = String::from("My apple’s quite tasty.\n");
    assert_eq!(result, expected);

    let markdown = r#"
testing, testing one, two, three, four, five, six, seven, eight, nine,  ten, eleven
"#;

    let result = parse_markdown_to_plaintext(markdown, ParseMarkdownOptions::default());
    let expected = String::from(
        "testing, testing one, two, three, four, five, six, seven, eight, nine,\nten, eleven\n",
    );
    assert_eq!(result, expected);

    let markdown =
        r#"<abbr>CLI<tool-tip inert role="tooltip">Command Line Interface</tool-tip></abbr>"#;
    let result = parse_markdown_to_plaintext(markdown, ParseMarkdownOptions::default());
    let expected = String::from("CLI\n");
    assert_eq!(result, expected);
}

#[test]
pub fn parse_markdown_to_plaintext_applies_canonical_root_url() {
    let markdown = "[Contact us](/contact) to find out more.";

    let mut options = ParseMarkdownOptions::default();
    options.canonical_root_url(Some("https://example.com"));
    let result = parse_markdown_to_plaintext(markdown, options);
    let expected = String::from("Contact us (https://example.com/contact) to find out more.\n");
    assert_eq!(result, expected);
}

#[test]
pub fn parse_markdown_to_plaintext_outputs_relative_urls_when_canonical_root_url_absent() {
    let markdown = "[Contact us](/contact) to find out more.";

    let mut options = ParseMarkdownOptions::default();
    options.canonical_root_url(None);
    let result = parse_markdown_to_plaintext(markdown, options);
    let expected = String::from("Contact us (/contact) to find out more.\n");
    assert_eq!(result, expected);
}

#[test]
pub fn test_slugified_title() {
    let title = "Heading One";
    assert_eq!(slugified_title(title), "heading-one");

    let title = "🌟 Heading Two";
    assert_eq!(slugified_title(title), "*-heading-two");

    let title = "💫 Heading Three";
    assert_eq!(slugified_title(title), "dizzy-heading-three");

    let title = "Heading Four!";
    assert_eq!(slugified_title(title), "heading-four");
}
