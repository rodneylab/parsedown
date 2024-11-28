use crate::text::format_line;

#[test]
fn format_line_replaces_inner_apostrophe() {
    let line = "My apple's quite tasty.";
    let result = format_line(line);
    assert_eq!(result, "My apple’s quite tasty.");
}

#[test]
fn test_line_replaces_outer_apostrophe() {
    let line = "My trees' apples are tasty.";
    let result = format_line(line);
    assert_eq!(result, "My trees’ apples are tasty.");
}

#[test]
fn test_line_adds_double_smart_quotes() {
    let line = r#"The person said "My apple is quite tasty.""#;
    let result = format_line(line);
    assert_eq!(result, r#"The person said “My apple is quite tasty.”"#);
}

#[test]
fn test_line_adds_single_smart_quotes() {
    let line = "My apple is quite 'tasty'.";
    let result = format_line(line);
    assert_eq!(result, "My apple is quite ‘tasty’.");
}

#[test]
fn test_line_replaces_unmatched_single_double_quote_pairs() {
    let line = r#"My apple is quite 'tasty"."#;
    let result = format_line(line);
    assert_eq!(result, "My apple is quite ‘tasty”.");
}

#[test]
fn test_line_does_nothing_when_line_has_no_quotes_or_apostrophes() {
    let line = "My apple is quite tasty.";
    let result = format_line(line);
    assert_eq!(result, "My apple is quite tasty.");
}
