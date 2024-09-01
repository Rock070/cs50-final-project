use validator::ValidateUrl;

#[derive(Debug, Clone)]
pub struct Url(pub String);

#[derive(Debug, Clone)]
pub struct HashPath(pub String);

#[derive(Debug, Clone)]
pub struct UrlPath(pub String);

impl Url {
    pub fn parse_url(url: String) -> Result<Self, String> {
        if ValidateUrl::validate_url(&url) {
            return Ok(Self(url));
        }

        Err("Invalid URL".to_string())
    }
}

impl HashPath {
    pub fn parse_path(path: String) -> Result<Self, String> {
        if path
            .chars()
            .all(|c| c.is_ascii_alphabetic() || c.is_ascii_digit())
        {
            return Ok(Self(path));
        }

        Err("Invalid URL path".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_valid_url() {
        let url = "https://www.google.com".to_string();
        let result = Url::parse_url(url);
        assert!(result.is_ok());
    }

    #[test]
    fn test_new_invalid_url() {
        let url1 = "invalid_url".to_string();
        let url2 = "google.com".to_string();
        let url3 = "https/s/google.com".to_string();
        assert!(Url::parse_url(url1).is_err());
        assert!(Url::parse_url(url2).is_err());
        assert!(Url::parse_url(url3).is_err());
    }

    #[test]
    fn test_path() {
        assert!(HashPath::parse_path(String::from("abc")).is_ok());
        assert!(HashPath::parse_path(String::from("a1b2")).is_ok());
        assert!(HashPath::parse_path(String::from("a1b2!")).is_err());
    }
}
