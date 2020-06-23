pub mod errors;
pub mod responses;
use crate::models::user;
use crate::models::user::{NewUserRequest, UserLoginRequest, UserResponse};

use crate::lib::Database;

use rocket_contrib::json::Json;

use rocket::response::status;

// use rocket::http::Status;

use diesel::result::DatabaseErrorKind::UniqueViolation;
use diesel::result::Error::DatabaseError;

use self::responses::UserRegistrationResponse;

pub const BASE_PATH: &str = "/auth";

#[post("/register", format = "json", data = "<new_user>")]
pub fn register(conn: Database, new_user: Json<NewUserRequest>) -> UserRegistrationResponse {
    match user::create(
        &*conn,
        new_user.username.to_string(),
        new_user.password.to_string(),
        new_user.profile.to_string(),
    ) {
        Ok(user) => {
            println!("Database insert successful, new user {:?}", user);
            UserRegistrationResponse::Created(UserResponse {
                userId: user.id,
                username: user.username,
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
            UserRegistrationResponse::Created(UserResponse {
                userId: 0,
                username: new_user.username.to_string(),
                profile: new_user.profile.to_string(),
            })
            // UserRegistrationResponse::UniqueViolation(errors::UniqueViolation {
            //     table_name: table_name.to_string(),
            //     constraint_name: constraint_name.to_string(),
            // })
        }
        Err(_) => {
            UserRegistrationResponse::UnhandledException(errors::UnhandledException {
                message: "An unhandled error occurred with mod auth::register".to_string(),
            })
            // println!("An unhandled error occurred with mod auth::register")
        }
    }
    // println!("result, {:?}",result);
}
#[post("/login", format = "json", data = "<user>")]
pub fn login(conn: Database, user: Json<UserLoginRequest>) -> String {
    println!("the user {:?}", user);
    return match user::get_by_username(&conn, &user.username) {
        Ok(user_list) => {
            println!("got user list, {:?}", user_list);
            match user_list.first() {
                Some(db_user) => {
                    println!("comparing password!");
                    if (user.password == db_user.password) {
                        String::from("welcome!")
                    } else {
                        String::from("invalid username or password")
                    }
                }
                None => String::from("invalid username or password"),
            }
        }
        Err(db_error) => format!("error {:?}", db_error),
    };
}
#[get("/logout")]
pub fn logout() -> &'static str {
    "ok.. logging out now."
}
// #[get("/session")]
// pub fn get_auth_session() -> &'static str {
//     "here is your auth session: "
// }
// #[post("/session")]
// pub fn start_auth_session() -> &'static str {
//     "ok.. starting auth session"
// }
