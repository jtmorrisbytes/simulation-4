use crate::schema::users;
use serde::{Serialize,Deserialize};

pub trait Updatable {
  fn update(id:u32,username:String,profile_pic:String)->User;
  // fn updatePassword(id:u32,new_password:String)->();
}
pub trait Createable {
  fn create(username:String,password:String,profile:String);
}
pub trait Readable {
  // fn getAll();
  fn getOne();
}


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
impl Createable for User {
  fn create(username:String,password:String,profile:String){
    use crate::schema::users::dsl::*;
    // insert_into(users);
    println!("User.create requested")
  }
}
impl Readable for User {
  fn getOne(){
    println!("get User requested")
  }
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