use serde::Deserialize;
use validator::Validate;
use validator::ValidateUrl;
#[derive(Debug, Deserialize, Validate)]
pub struct HashUrlBody {
    #[validate(url)]
    pub url: String,
}

impl HashUrlBody {
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
        let result = HashUrlBody::new(url);
        assert!(result.is_ok());
    }

    #[test]
    fn test_new_invalid_url() {
        let url1 = "invalid_url".to_string();
        let url2 = "google.com".to_string();
        assert!(HashUrlBody::new(url1).is_err());
        assert!(HashUrlBody::new(url2).is_err());
    }
}
