pub mod errors;
pub mod responses;
use crate::models::user;
use crate::models::user::{NewUserRequest, UserResponse};

use rocket_contrib::json::Json;

use rocket::response;

// use rocket::http::Status;

use diesel::result::DatabaseErrorKind::UniqueViolation;
use diesel::result::Error::DatabaseError;

use self::responses::UserRegistrationResponse;

pub const BASE_PATH: &str = "/auth";
#[get("/login")]
pub fn login() -> &'static str {
    "hello from auth login"
}
#[post("/register", format = "json", data = "<new_user>")]
pub fn register(
    new_user: Json<NewUserRequest>,
) -> Result<UserResponse, errors::UserRegistrationError> {
    match user::create(
        new_user.username.to_string(),
        new_user.password.to_string(),
        new_user.profile.to_string(),
    ) {
        Ok(user) => {
            println!("Database insert successful, new user {:?}", user);
            Ok(UserResponse {
                userId: user.id,
                username:user.username,
                profile: user.profile_pic.unwrap_or("None".to_string()),
            })
        }
        Err(DatabaseError(UniqueViolation, message)) => {
            let table_name = message.table_name().unwrap_or("null");
            let constraint_name = message.constraint_name().unwrap_or("null");
            println!(
                "auth::register : A UniqueViolation occurred, table '{0}', constraint '{1}'",
                table_name, constraint_name
            );
            Err(errors::UserRegistrationError::UniqueViolation(
                errors::UniqueViolation {
                    table_name: table_name.to_string(),
                    constraint_name: constraint_name.to_string(),
                },
            ))
        }
        Err(_) => {
            Err(errors::UserRegistrationError::UnhandledException(
                errors::UnhandledException {
                    message: "An unhandled error occurred with mod auth::register".to_string(),
                },
            ))
            // println!("An unhandled error occurred with mod auth::register")
        }
        _ => {
            println!("auth::register DB match was exhausted with an unhandled case");

            Err(errors::UserRegistrationError::UnhandledException(
                errors::UnhandledException {
                    message:"auth::register match was completely exhausted unhandled case after handling errors".to_string()
                }
            ))
        }
    }
    // println!("result, {:?}",result);
}
#[get("/logout")]
pub fn logout() -> &'static str {
    "ok.. logging out now."
}
#[get("/session")]
pub fn get_auth_session() -> &'static str {
    "here is your auth session: "
}
#[post("/session")]
pub fn start_auth_session() -> &'static str {
    "ok.. starting auth session"
}
