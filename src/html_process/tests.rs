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
    let result = process_html("<a href=\"https://example.com\">Example</a>", None, None);
    let expected = r#"<a href="https://example.com" target="_blank" rel="nofollow noopener noreferrer">Example</a>"#;
    assert_eq!(result, expected);

    let result = process_html(
        "<a href=\"/pathname?utm=123#anchor\">Example</a>",
        None,
        None,
    );
    let expected = r#"<a href="/pathname?utm=123#anchor">Example</a>"#;
    assert_eq!(result, expected);

    let result = process_html("<h2>Heading</h2>", None, None);
    let expected = "<h2>Heading</h2>";
    assert_eq!(result, expected);

    let result = process_html("<h2 id=\"heading\">Heading</h2>", None, None);
    let expected =
        "<h2 id=\"heading\">Heading <a href=\"#heading\" class=\"heading-anchor\">#</a></h2>";
    assert_eq!(result, expected);

    let result = process_html("<h3 id=\"heading\">Heading</h3>", None, None);
    let expected = "<h3 id=\"heading\">Heading</h3>";
    assert_eq!(result, expected);
}

#[test]
fn test_relative_url() {
    assert!(relative_url("/about.html"));
    assert!(relative_url("#some-id"));
    assert_eq!(relative_url("https://example.com"), false);
}

#[test]
fn search_html_highlight_requested_term() {
    let result = process_html(
        r#"<h2>Heading</h2><p>Nobody likes maple in their apple flavoured Snapple. APPLE</p><p>Paragraph with no matches</p><p>Paragraph which mentions apples again</p>"#,
        None,
        Some("apple"),
    )
    .to_string();
    let expected = r#"<h2>Heading</h2><p>Nobody likes maple in their <mark id="search-match">apple</mark> flavoured Sn<mark>apple</mark>. <mark>APPLE</mark></p><p>Paragraph with no matches</p><p>Paragraph which mentions <mark>apple</mark>s again</p>"#;
    assert_eq!(result, expected);
}

#[test]
fn search_html_highlight_requested_nested_term() {
    let result = process_html(
        r#"<h2>Heading</h2><section><div><p>Nobody likes maple in their apple flavoured Snapple. APPLE</p><p>Paragraph with no matches</p><p>Paragraph which mentions apples again</p></div></section>"#,
        None,
        Some("apple"),
    )
    .to_string();
    let expected = r#"<h2>Heading</h2><section><div><p>Nobody likes maple in their <mark id="search-match">apple</mark> flavoured Sn<mark>apple</mark>. <mark>APPLE</mark></p><p>Paragraph with no matches</p><p>Paragraph which mentions <mark>apple</mark>s again</p></div></section>"#;
    assert_eq!(result, expected);
}

#[test]
fn search_html_matches_on_multiple_terms() {
    let result = process_html(
        r#"<h2>Heading</h2><p>Nobody likes maple in their apple flavoured Snapple. APPLE</p><p>Paragraph with no matches</p><p>Paragraph which mentions apples again</p>"#,
        None,
        Some("apple flavour"),
    )
    .to_string();
    let expected = r#"<h2>Heading</h2><p>Nobody likes maple in their <mark id="search-match">apple</mark> <mark>flavour</mark>ed Sn<mark>apple</mark>. <mark>APPLE</mark></p><p>Paragraph with no matches</p><p>Paragraph which mentions <mark>apple</mark>s again</p>"#;
    assert_eq!(result, expected);
}

#[test]
fn search_html_highlight_does_nothing_when_there_are_no_matches() {
    let result = process_html(
        r#"<h2>Heading</h2><p>Nobody likes maple in their apple flavoured Snapple. APPLE</p>"#,
        None,
        Some("nonsense"),
    )
    .to_string();
    let expected =
        r#"<h2>Heading</h2><p>Nobody likes maple in their apple flavoured Snapple. APPLE</p>"#;
    assert_eq!(result, expected);
}
