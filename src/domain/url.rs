use validator::Validate;
use validator::ValidateUrl;

#[derive(Debug, Validate)]
pub struct HashUrl {
    #[validate(url)]
    pub url: String,
}

impl HashUrl {
    pub fn new(url: String) -> Result<Self, String> {
        if ValidateUrl::validate_url(&url) {
            return Ok(Self { url });
        }

        Err("Invalid URL".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_valid_url() {
        let url = "https://www.google.com".to_string();
        let result = HashUrl::new(url);
        assert!(result.is_ok());
    }

    #[test]
    fn test_new_invalid_url() {
        let url1 = "invalid_url".to_string();
        let url2 = "google.com".to_string();
        let url3 = "https:asd/s/google.com".to_string();
        assert!(HashUrl::new(url1).is_err());
        assert!(HashUrl::new(url2).is_err());
        assert!(HashUrl::new(url3).is_err());
    }
}
