use crate::markdown::{parse_markdown_to_html, slugified_title};

#[test]
fn test_parse_markdown_to_html() {
    let markdown = r#"
hello
=====

* alpha
* beta
"#;

    let result = parse_markdown_to_html(markdown).ok();
    let expected = Some(String::from(
        r#"<h1 id="hello">hello</h1>
<ul>
<li>alpha</li>
<li>beta</li>
</ul>
"#,
    ));
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
