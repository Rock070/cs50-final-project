use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterData {
  #[validate(length(min = 1))]
  pub username: String,
  #[validate(length(min = 1))]
  pub password: String,
  #[validate(email)]
  pub email: String,
}
