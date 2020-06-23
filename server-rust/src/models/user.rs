use crate::schema::users;
use rocket::request::Request;
use rocket::response::{self, Responder};
use rocket_contrib::databases::diesel::prelude::*;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Debug)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    /*
      username & password will have VARCHAR(20) implemented
      when diesel is implemented.
      for now, it will be unlimited

    */
    pub username: String,
    pub password: String,
    // profile image
    pub profile_pic: Option<String>,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    username: String,
    password: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct NewUserRequest {
    pub username: String,
    pub password: String,
    pub profile: String,
}
#[derive(Serialize, Debug)]
#[allow(non_snake_case)]
pub struct UserResponse {
    pub username: String,
    pub profile: String,
    pub userId: i32,
}
impl<'r> Responder<'r> for UserResponse {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Json(self).respond_to(&req)
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserLoginRequest {
    pub username: String,
    pub password: String,
}
// #[allow(non_snake_case)]
// #[derive(Deserialize)]
// pub struct UserLoginResponse {
//     profile: String,
//     userId: u32,
// }
pub fn create(
    conn: &PgConnection,
    new_username: String,
    new_password: String,
    profile: String,
) -> Result<User, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    // estabish connection
    let insert_result = diesel::insert_into(users)
        .values((
            username.eq(new_username.to_string()),
            password.eq(new_password.to_string()),
            profile_pic.eq(profile),
        ))
        .returning((id, username, password, profile_pic))
        .get_results(conn);
    match insert_result {
        Ok(inserted_rows) => match inserted_rows.into_iter().next() {
            Some(user) => Ok(user),
            None => Ok(User {
                id: -1,
                username: "error".to_string(),
                password: "error".to_string(),
                profile_pic: None,
            }),
        },
        Err(insert_error) => Err(insert_error),
    }
    // result
}
pub fn get_by_username(
    conn: &PgConnection,
    the_username: &str,
) -> Result<Vec<User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    users
        .filter(username.eq(the_username.to_string()))
        .get_results::<User>(conn)
}
