use crate::markdown::{
    parse_markdown_to_html, parse_markdown_to_plaintext, reading_time_from_words, slugified_title,
    words,
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

    let result = if let Some((result, _)) = parse_markdown_to_html(markdown).ok() {
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

    let result = parse_markdown_to_plaintext(markdown);
    let expected = String::from("🧑🏽\u{200d}🍳 Pick of the Month — vanilla-extract\n");
    assert_eq!(result, expected);

    let markdown = r#"
testing, testing one, two, three, four, five, six, seven, eight, nine,  ten, eleven
"#;

    let result = parse_markdown_to_plaintext(markdown);
    let expected = String::from(
        "testing, testing one, two, three, four, five, six, seven, eight, nine,\nten, eleven\n",
    );
    assert_eq!(result, expected);

    let markdown = r#"
hello
=====

It's me

* alpha
* beta
"#;

    let markdown =
        r#"<abbr>CLI<tool-tip inert role="tooltip">Command Line Interface</tool-tip></abbr>"#;
    let result = parse_markdown_to_plaintext(markdown);
    let expected = String::from("CLI\n");
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
