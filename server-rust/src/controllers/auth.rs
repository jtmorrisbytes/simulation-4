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

use bcrypt::{hash, verify};

use self::responses::UserRegistrationResponse;

pub const BASE_PATH: &str = "/auth";

#[post("/register", format = "json", data = "<new_user>")]
pub fn register(conn: Database, new_user: Json<NewUserRequest>) -> UserRegistrationResponse {
    let hash = match bcrypt::hash(&new_user.password, 14) {
        Ok(hash) => hash,
        Err(_) => {
            println!("something went wrong while hashing");
            String::from("")
        }
    };
    println!("hash {0}, len {1}", hash, hash.len());
    match user::create(
        &*conn,
        new_user.username.to_string(),
        hash,
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
        Err(some_err) => {
            println!("An unhandled error occurred, {:?}", some_err);
            UserRegistrationResponse::UnhandledException(errors::UnhandledException {
                message: "An unhandled error occurred with mod auth::register".to_string(),
            })
            // println!("An unhandled error occurred with mod auth::register")
        }
    }
    // println!("result, {:?}",result);
}
#[post("/login", format = "json", data = "<user>")]
pub fn login(conn: Database, user: Json<UserLoginRequest>) -> responses::UserLoginResponse {
    println!("the user {:?}", user);
    return match user::get_by_username(&conn, &user.username) {
        Ok(user_list) => {
            println!("got user list, {:?}", user_list);
            match user_list.first() {
                Some(db_user) => {
                    println!("comparing password!");
                    if bcrypt::verify(&user.password, &db_user.password).unwrap_or(false) {
                        responses::UserLoginResponse::Ok(UserResponse {
                            userId: db_user.id,
                            username: db_user.username.to_string(),
                            profile: db_user
                                .profile_pic
                                .as_ref()
                                .unwrap_or(&String::from("null"))
                                .to_string(),
                        })
                    } else {
                        responses::UserLoginResponse::NotAuthorized(
                            errors::InvalidUsernamePassword::new(),
                        )
                    }
                }
                None => responses::UserLoginResponse::NotAuthorized(
                    errors::InvalidUsernamePassword::new(),
                ),
            }
        }
        Err(db_error) => {
            responses::UserLoginResponse::DatabaseError(format!("error {:?}", db_error))
        }
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
