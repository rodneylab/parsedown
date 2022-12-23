use crate::markdown::{parse_markdown_to_html, slugified_title, words};

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
