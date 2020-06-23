pub mod errors;
pub mod responses;
use crate::models::user;
use crate::models::user::{NewUserRequest, UserLoginRequest, UserResponse};

use crate::lib::Database;

use rocket_contrib::json::Json;

use diesel::result::DatabaseErrorKind::{UniqueViolation, __Unknown};
use diesel::result::Error::DatabaseError;

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
        Err(DatabaseError(__Unknown, error)) => {
            println!(
                "Unhandled database Error!, for table '{0:?}', column {1:?}, constraint {2:?}, message {3:?}, details {4:?}",
                error.table_name(),
                error.column_name(),
                error.constraint_name(),
                error.message(),
                error.details()
            );
            UserRegistrationResponse::UnhandledException(errors::UnhandledException {
                message: error.message().to_string(),
            })
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
#[allow(dead_code)]
fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .attach(Database::fairing())
        .mount(BASE_PATH, routes![register, login])
}
#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::http::ContentType;
    use rocket::http::Status;
    use rocket::local::Client;
    extern crate rand;
    use crate::models::user::NewUserRequest;
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};
    use rocket_contrib::json::Json;
    // use serde::{Deserialize, Serialize};
    // extern crate serde_json;
    #[test]
    pub fn test_register() {
        let client = Client::new(rocket()).expect("invalid rocket instance");
        // let username_sample =
        let good_username = format!(
            "test_user_{}",
            thread_rng()
                .sample_iter(&Alphanumeric)
                .take(10)
                .collect::<String>()
        );
        let good_password = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(20)
            .collect::<String>();
        println!(
            "good username: len {0} '{1}', good password: len {2},{3}",
            good_username,
            good_username.len(),
            good_password,
            good_password.len()
        );
        let profile = thread_rng().sample_iter(&Alphanumeric).take(500).collect();
        let test_user = NewUserRequest {
            username: good_username,
            password: good_password,
            profile,
        };
        let response = client
            .post("/auth/register")
            .header(ContentType::JSON)
            .body(serde_json::to_string(&test_user).unwrap_or(String::from("")))
            .dispatch();
        assert_eq!(response.status(), Status::Created);
        assert_eq!(
            response.content_type().expect("Content type required!"),
            ContentType::JSON
        );
        // let response = client.post("/auth/register").body(Json(test_user).respond_to(req: &Request))
        println!("response from rocket {:?}", response)
    }
}
