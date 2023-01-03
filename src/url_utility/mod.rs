use url::Url;

pub fn relative_url(url: &str) -> bool {
    match Url::parse(url) {
        Ok(_) => false,
        Err(url::ParseError::RelativeUrlWithoutBase) => true,
        Err(_) => false,
    }
}
