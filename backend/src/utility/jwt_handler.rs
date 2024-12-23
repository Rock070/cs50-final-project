use chrono::Utc;

use crate::ApplicationSetting;
use jsonwebtoken::{
    decode, encode, errors::Result, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub aud: String, // Optional. Audience (who or what the token is intended for)
    exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: usize, // Optional. Issued at (as UTC timestamp) (default = now)
    iss: String, // Optional. Issuer (who issued the token)
    nbf: usize, // Optional. Not Before (as UTC timestamp)
    sub: String, // Optional. Subject (whom token refers to)
}

#[derive(Debug, Clone)]
pub struct JwtHandler {
    pub private_key: Secret<String>,
    pub header: Header,
    pub public_key: String,
    pub expiration_minutes: i64,
}

impl JwtHandler {
    pub fn create_token(self, setting: &ApplicationSetting, user_name: &str) -> Result<String> {
        let claims = Claims {
            aud: user_name.to_owned(),
            exp: (Utc::now() + chrono::Duration::minutes(self.expiration_minutes)).timestamp()
                as usize,
            iat: Utc::now().timestamp() as usize,
            iss: format!("{} - {}", setting.name, setting.owner),
            nbf: Utc::now().timestamp() as usize,
            sub: setting.name.to_owned(),
        };
        let private_key = self.private_key.expose_secret();

        encode(
            &self.header,
            &claims,
            &EncodingKey::from_rsa_pem(private_key.as_bytes())?,
        )
    }

    pub fn decode_token(self, setting: &ApplicationSetting, token: String) -> Result<Claims> {
        // 非對稱加密
        let mut validation = Validation::new(Algorithm::RS256);

        validation.sub = Some(setting.name.to_owned());
        validation.validate_aud = false;

        decode::<Claims>(
            &token,
            &DecodingKey::from_rsa_pem(self.public_key.as_bytes())?,
            &validation,
        )
        .map(|data| data.claims)
    }
}

// {
//     required_spec_claims: {"exp"},
//     leeway: 60,
//     reject_tokens_expiring_in_less_than: 0,
//     validate_exp: true,
//     validate_nbf: false,
//     validate_aud: true,
//     aud: None,
//     iss: None,
//     sub: None,
//     algorithms: [RS256],
//     validate_signature: true
// }
