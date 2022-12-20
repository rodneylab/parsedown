use crate::html_process::relative_url;

#[test]
fn test_relative_url() {
    assert!(relative_url("/about.html"));
    assert!(relative_url("#some-id"));
    assert_eq!(relative_url("https://example.com"), false);
}
