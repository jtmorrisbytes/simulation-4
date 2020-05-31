use crate::lib;
use crate::schema::users;
use diesel::prelude::*;
use rocket::request::Request;
use rocket::response::{self, Responder};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Queryable,Insertable,Debug)]
#[table_name="users"]
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
pub fn create(
    new_username: String,
    new_password: String,
    profile: String,
) -> Result<User, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    // estabish connection
    let conn = lib::establish_connection();
    let insert_result = diesel::insert_into(users)
        .values((
            username.eq(new_username.to_string()),
            password.eq(new_password.to_string()),
            profile_pic.eq(profile),
        ))
        .returning((id,username,password,profile_pic))
        .get_results(&conn);
    match insert_result {
      Ok(inserted_rows) => {

        match inserted_rows.into_iter().next(){
          Some(user)=>Ok(user),
          None =>Ok(User{id:-1,username:"error".to_string(),password:"error".to_string(),profile_pic:None})
        }
      },  
      Err(insert_error)=>{Err(insert_error)}
    }
    // result
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    username: String,
    password: String,
}
#[derive(Deserialize)]
pub struct NewUserRequest {
    pub username: String,
    pub password: String,
    pub profile: String,
}
#[derive(Serialize)]
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

#[derive(Serialize)]
pub struct UserLoginRequest {
    username: String,
    password: String,
}
#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct UserLoginResponse {
    profile: String,
    userId: u32,
}
