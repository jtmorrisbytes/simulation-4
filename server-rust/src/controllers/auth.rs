use crate::models::user::{NewUserRequest,UserResponse};
use crate::models::user;
use rocket_contrib::json::Json;
pub const BASE_PATH: &'static str = "/auth";
#[get("/login")]
pub fn login() -> &'static str {
    "hello from auth login"
}
#[post("/register", format="json", data="<new_user>")]
pub fn register(new_user: Json<NewUserRequest>) -> Json<UserResponse> {
    let result = user::create(
        new_user.username.to_string(),
        new_user.password.to_string(),
        new_user.profile.to_string());
    match result {
        Ok(num_rows)=>{println!("{} rows inserted",num_rows)},
        Err(err)=>{println!("need to return a user response, {:?}",err)}
        _=>{println!("Something happened")}
    }
    Json(
        UserResponse {
            userId:0,
            username:new_user.username.to_string(),
            profile:new_user.profile.to_string()
    })
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
