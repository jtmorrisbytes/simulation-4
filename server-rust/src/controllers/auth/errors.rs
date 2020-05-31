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
#[derive(Deserialize, Serialize, Debug)]
pub struct UnhandledException {
    pub message: String,
}

impl<'r> Responder<'r> for UniqueViolation {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Json(self).respond_to(&req)
    }
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
// impl<'r> Responder<'r> for UserRegistrationError {
//   fn respond_to(self,req:&Request) -> response::Result<'r>{
//     let mut response = Response::build();
//     match self {
//       UserRegistrationError::UniqueViolation(unique_violation) =>{
//         Ok(response.status(Status.BadRequest).merge(Json(unique_violation).respond_to(&req)))
//       },
//       UserRegistrationError::UnhandledException(exception) =>{}
//     }
//   }
// }
