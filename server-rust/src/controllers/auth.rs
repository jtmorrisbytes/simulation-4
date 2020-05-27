use crate::models::user::{NewUserRequest,UserResponse};
use rocket_contrib::json::Json;
pub const BASE_PATH: &'static str = "/auth";
#[get("/login")]
pub fn login() -> &'static str {
    "hello from auth login"
}
#[post("/register", format="json", data="<new_user>")]
pub fn register(new_user: Json<NewUserRequest>) -> Json<UserResponse> {
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
