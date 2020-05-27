use crate::schema::users;
use serde::{Serialize,Deserialize};
use diesel::prelude::*;
use crate::lib;







pub struct User {
  id:u32,
  /*
    username & password will have VARCHAR(20) implemented
    when diesel is implemented.
    for now, it will be unlimited

  */
  username:String,
  password:String,
  // profile image
  profile_pic:String
}
pub fn create(new_username:String,new_password:String,profile:String)->Result<usize,diesel::result::Error>{
    use crate::schema::users::dsl::*;
    // estabish connection
    let conn = lib::establish_connection();
    diesel::insert_into(users)
    .values(
      (username.eq(new_username.to_string()),
      password.eq(new_password.to_string()),
      profile_pic.eq(profile))
    ).execute(&conn)
  }



#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser {
  username:String,
  password:String,
}
#[derive(Deserialize)]
pub struct NewUserRequest {
  pub username:String,
  pub password:String,
  pub profile:String,
}
#[derive(Serialize)]
#[allow(non_snake_case)]
pub struct UserResponse {
  pub username: String,
  pub profile:String,
  pub userId:u32
}


#[derive(Serialize)]
pub struct UserLoginRequest {
  username:String,
  password:String,
}
#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct UserLoginResponse {
  profile:String,
  userId:u32,
}