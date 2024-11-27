use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{alphanumeric1, multispace0},
    combinator::recognize,
    multi::many1_count,
    sequence::{delimited, pair},
    IResult,
};

#[derive(Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum InlineHTMLTagType {
    Opening(String),
    //SelfClosing,
    Closing(String),
}

fn parse_html_tag_content(line: &str) -> IResult<&str, (&str, &str)> {
    let (remainder, tag_content) = is_not(">/")(line)?;
    let (attributes, (tag_name, _space)) = pair(
        recognize(many1_count(alt((alphanumeric1, tag("-"))))),
        multispace0,
    )(tag_content)?;
    Ok((remainder, (tag_name, attributes)))
}

fn parse_closing_html_tag(line: &str) -> IResult<&str, (&str, &str, InlineHTMLTagType)> {
    let (remaining_line, (tag_name, tag_attributes)) =
        delimited(tag("</"), parse_html_tag_content, tag(">"))(line)?;
    Ok((
        remaining_line,
        (
            tag_name,
            tag_attributes,
            InlineHTMLTagType::Closing(tag_name.into()),
        ),
    ))
}

fn parse_opening_html_tag(line: &str) -> IResult<&str, (&str, &str, InlineHTMLTagType)> {
    let (remaining_line, (tag_name, tag_attributes)) =
        delimited(tag("<"), parse_html_tag_content, tag(">"))(line)?;
    Ok((
        remaining_line,
        (
            tag_name,
            tag_attributes,
            InlineHTMLTagType::Opening(tag_name.into()),
        ),
    ))
}

pub fn parse_node(html_node: &str) -> Option<InlineHTMLTagType> {
    match alt((parse_opening_html_tag, parse_closing_html_tag))(html_node) {
        Ok((_, (_, _, tag_type))) => Some(tag_type),
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        parse_closing_html_tag, parse_html_tag_content, parse_node, parse_opening_html_tag,
        InlineHTMLTagType,
    };

    #[test]
    pub fn parse_html_tag_content_parses_valid_html_tag_without_attributes() {
        // arrange
        let tag_content = "abbr";

        // act
        let result = parse_html_tag_content(tag_content);

        // assert
        assert_eq!(result, Ok(("", ("abbr", ""))));
    }

    #[test]
    pub fn parse_html_tag_content_parses_valid_html_tag_with_attributes() {
        // arrange
        let tag_content = r#"tool-tip inert role="tooltip""#;

        // act
        let result = parse_html_tag_content(tag_content);

        // assert
        assert_eq!(result, Ok(("", ("tool-tip", r#"inert role="tooltip""#))));
    }

    #[test]
    pub fn parse_opening_html_tag_parses_valid_html_tag_without_attributes() {
        // arrange
        let tag = "<abbr>";

        // act
        let result = parse_opening_html_tag(tag);

        // assert
        assert_eq!(
            result,
            Ok((
                "",
                ("abbr", "", InlineHTMLTagType::Opening(String::from("abbr")))
            ))
        );
    }

    #[test]
    pub fn parse_opening_html_tag_parses_valid_html_tag_with_attributes() {
        // arrange
        let tag = r#"<tool-tip inert role="tooltip">"#;

        // act
        let result = parse_opening_html_tag(tag);

        // assert
        assert_eq!(
            result,
            Ok((
                "",
                (
                    "tool-tip",
                    r#"inert role="tooltip""#,
                    InlineHTMLTagType::Opening(String::from("tool-tip"))
                )
            ))
        );
    }

    #[test]
    pub fn parse_closing_html_tag_parses_valid_html_tag() {
        // arrange
        let tag = "</tool-tip>";

        // act
        let result = parse_closing_html_tag(tag);

        // assert
        assert_eq!(
            result,
            Ok((
                "",
                (
                    "tool-tip",
                    "",
                    InlineHTMLTagType::Closing(String::from("tool-tip"))
                )
            ))
        );
    }

    #[test]
    pub fn parse_node_parses_valid_opening_html_tag() {
        // arrange
        let tag = "</tool-tip>";

        // act
        let result = parse_node(tag);

        // assert
        assert_eq!(
            result,
            Some(InlineHTMLTagType::Closing(String::from("tool-tip")))
        );
    }

    #[test]
    pub fn parse_node_parses_valid_closing_html_tag() {
        // arrange
        let tag = r#"<tool-tip inert role="tooltip">"#;

        // act
        let result = parse_node(tag);

        // assert
        assert_eq!(
            result,
            Some(InlineHTMLTagType::Opening(String::from("tool-tip")))
        );
    }

    #[test]
    pub fn parse_node_returns_none_for_invalid_html_tag() {
        // arrange
        let tag = "<tool-tip";

        // act
        let result = parse_node(tag);

        // assert
        assert_eq!(result, None);
    }
}
