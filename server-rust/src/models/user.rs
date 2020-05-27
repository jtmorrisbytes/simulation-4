use super::schema::users;
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
#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser {
  username:String,
  password:String,
}
pub struct NewUserResponse {
  username: String,
  profile:String,
  userId:u32
}