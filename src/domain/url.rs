use validator::ValidateUrl;

#[derive(Debug, Clone)]
pub struct Url(pub String);

impl Url {
    pub fn parse(url: String) -> Result<Self, String> {
        if ValidateUrl::validate_url(&url) {
            return Ok(Self(url));
        }

        Err("Invalid URL".to_string())
    }
}

#[derive(Debug, Clone)]
pub struct HashUrl {
    pub url: Url,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_valid_url() {
        let url = "https://www.google.com".to_string();
        let result = Url::parse(url);
        assert!(result.is_ok());
    }

    #[test]
    fn test_new_invalid_url() {
        let url1 = "invalid_url".to_string();
        let url2 = "google.com".to_string();
        let url3 = "https:asd/s/google.com".to_string();
        assert!(Url::parse(url1).is_err());
        assert!(Url::parse(url2).is_err());
        assert!(Url::parse(url3).is_err());
    }
}
