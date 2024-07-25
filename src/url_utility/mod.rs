use url::Url;

pub fn relative_url(url: &str) -> bool {
    match Url::parse(url) {
        Err(url::ParseError::RelativeUrlWithoutBase) => true,
        Ok(_) | Err(_) => false,
    }
}
