use validator::Validate;
use uuid::Uuid;

use crate::{
  application::AppState,
  domain::RegisterData,
  entity::users,
};

use argon2::{
  password_hash::{
      rand_core::OsRng,
      PasswordHasher, SaltString
  },
  Argon2
};

use sea_orm::{
  ActiveValue, 
  ColumnTrait, 
  EntityTrait,
  QueryFilter
};

use axum::{
  extract::{State, Json},
  http::StatusCode
};

pub async fn user_register(state: State<AppState>, Json(payload): Json<RegisterData>) -> Result<(StatusCode, Json<String>), (StatusCode, Json<String>)> {
  let register_data = RegisterData {
    username: payload.username.clone(),
    password: payload.password,
    email: payload.email,
  };
  
  match register_data.validate() {
        Ok(_) => {
          // TODO: 阻擋重複使用者
          let password = register_data.password.as_bytes();
          let salt = SaltString::generate(&mut OsRng);
          let argon2 = Argon2::default();
          let password_hash = argon2.hash_password(password, &salt).unwrap().to_string();

          let username = register_data.username.clone();

          let user = users::Entity::find()
            .filter(users::Column::Username.eq(username))
            .one(&state.database)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())))?;

          if user.is_none() {
            let register_data_model = users::ActiveModel {
              id: ActiveValue::Set(Uuid::new_v4()),
              username: ActiveValue::Set(register_data.username),
              password: ActiveValue::Set(password_hash),
              email: ActiveValue::Set(register_data.email),
              salt: ActiveValue::Set(salt.to_string()),
              ..Default::default()
            };    
            
            users::Entity::insert(register_data_model)
              .exec(&state.database)
              .await
              .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())))?;

            return Ok((StatusCode::OK, Json(r#"{"message": "register successfully"}"#.to_string())));
          } else {
            return Err((StatusCode::BAD_REQUEST, Json(r#"{"message": "username already exists"}"#.to_string())));
          }
        },
        Err(e) => return Err((StatusCode::BAD_REQUEST, Json(e.to_string())))
    }   
}