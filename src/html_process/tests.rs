use crate::html_process::{process_html, relative_url};

#[test]
fn test_process_html() {
    let result = process_html("<a href=\"https://example.com\">Example</a>");
    let expected =
        "<a href=\"https://example.com\" rel=\"nofollow noopener noreferrer\">Example</a>";
    assert_eq!(result, expected);

    let result = process_html("<a href=\"/pathname?utm=123#anchor\">Example</a>");
    let expected = "<a href=\"/pathname?utm=123#anchor\">Example</a>";
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
