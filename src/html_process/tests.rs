use crate::html_process::{process_html, relative_url, Builder};

#[test]
fn test_builder_process() {
    let result = Builder::new()
        .canonical_root_url(Some("https://example.com"))
        .process(r#"<a href="/about-us">About</a>"#)
        .to_string();
    let expected = r#"<a href="https://example.com/about-us">About</a>"#;
    assert_eq!(result, expected);

    let result = Builder::new()
        .process(r#"<a href="/about-us">About</a>"#)
        .to_string();
    let expected = r#"<a href="/about-us">About</a>"#;
    assert_eq!(result, expected);
}

#[test]
fn test_process_html() {
    let result = process_html("<a href=\"https://example.com\">Example</a>");
    let expected = r#"<a href="https://example.com" target="_blank" rel="nofollow noopener noreferrer">Example</a>"#;
    assert_eq!(result, expected);

    let result = process_html("<a href=\"/pathname?utm=123#anchor\">Example</a>");
    let expected = r#"<a href="/pathname?utm=123#anchor">Example</a>"#;
    assert_eq!(result, expected);

    let result = process_html("<h2>Heading</h2>");
    let expected = "<h2>Heading</h2>";
    assert_eq!(result, expected);

    let result = process_html("<h2 id=\"heading\">Heading</h2>");
    let expected =
        "<h2 id=\"heading\">Heading <a href=\"#heading\" class=\"heading-anchor\">#</a></h2>";
    assert_eq!(result, expected);

    let result = process_html("<h3 id=\"heading\">Heading</h3>");
    let expected = "<h3 id=\"heading\">Heading</h3>";
    assert_eq!(result, expected);
}

#[test]
fn test_relative_url() {
    assert!(relative_url("/about.html"));
    assert!(relative_url("#some-id"));
    assert_eq!(relative_url("https://example.com"), false);
}
