use super::errors::UserRegistrationError;
use crate::models::user::UserResponse;

use rocket_contrib::json::Json;

use rocket::request::Request;
use rocket::response::{self, Responder};

use rocket::http::Status;

// use rocket::http::ContentType;

pub enum UserRegistrationResponse {
    Sucess(UserResponse),
    Error(UserRegistrationError),
}
// impl<'r> Responder<'r> for UserRegistrationResponse {
//   fn respond_to(self,request: &Request) ->response::Result<'r>{
//     // let mut response = Response::build();
//     match self {
//       UserRegistrationResponse::Sucess(user_response)=>{
//        Json(user_response).respond_to(&request)
//       },
//       UserRegistrationResponse::Error(UserRegistrationError::UniqueViolation(violation_response))=>{
//        Ok(violation_response.respond_to(&request))
//       },
//       UserRegistrationResponse::Error(UserRegistrationError::UnhandledException(message))=>{
//         Json(message).respond_to(&request)
//       }
//     }
//   }
// }
