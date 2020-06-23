use crate::models::user::UserResponse;

use rocket::response::Responder;

use serde::Serialize;

use crate::auth::errors;

// use rocket::http::ContentType;
#[derive(Serialize, Responder, Debug)]
pub enum UserRegistrationResponse {
    #[response(status = 201)]
    Created(UserResponse),
    #[response(status = 500)]
    UnhandledException(errors::UnhandledException),
    #[response(status = 201)]
    UniqueViolation(errors::UniqueViolation),
}

#[derive(Responder, Serialize)]
pub enum UserLoginResponse {
    Ok(UserResponse),
    #[response(status = 401)]
    NotAuthorized(errors::InvalidUsernamePassword),
    #[response(status = 500)]
    DatabaseError(String),
}
