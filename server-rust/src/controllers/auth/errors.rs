use serde::{Deserialize, Serialize};

use rocket::request::Request;
use rocket::response::{self, /*Response,*/ Responder};
// use rocket::http::Status;

use rocket_contrib::json::Json;

#[derive(Serialize, Deserialize, Debug)]
pub struct UniqueViolation {
    pub table_name: String,
    pub constraint_name: String,
}
impl<'r> Responder<'r> for UniqueViolation {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Json(self).respond_to(&req)
    }
}
#[derive(Deserialize, Serialize, Debug)]
pub struct UnhandledException {
    pub message: String,
}
impl<'r> Responder<'r> for UnhandledException {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Json(self).respond_to(&req)
    }
}

#[derive(Deserialize, Serialize, Responder, Debug)]
pub enum UserRegistrationError {
    #[response(status = 400)]
    UniqueViolation(UniqueViolation),
    #[response(status = 500)]
    UnhandledException(UnhandledException),
}
#[derive(Responder, Serialize, Deserialize)]
#[response(status = 401)]
pub struct InvalidUsernamePassword {
    message: String,
}
impl InvalidUsernamePassword {
    pub fn new() -> Self {
        InvalidUsernamePassword {
            message: String::from("Invalid username or password"),
        }
    }
}

#[derive(Responder, Serialize)]
pub enum UserLoginError {
    NotAuthorized(InvalidUsernamePassword),
    #[response(status = 500)]
    DatabaseError(String),
}
