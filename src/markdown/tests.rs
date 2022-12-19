use crate::markdown::slugified_title;

#[test]
pub fn test_slugified_title() {
    let title = "Heading One";
    assert_eq!(slugified_title(title),"heading-one");

        let title = "🌟 Heading Two";
    assert_eq!(slugified_title(title),"*-heading-two");

        let title = "💫 Heading Three";
    assert_eq!(slugified_title(title),"dizzy-heading-three");

        let title = "Heading Four!";
    assert_eq!(slugified_title(title),"heading-four");

}
