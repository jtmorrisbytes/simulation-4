use super::models::users;

pub const BASE_PATH: &'static str = "/auth";
#[get("/login")]
pub fn login() -> &'static str {
    "hello from auth login"
}
#[get("/register")]
pub fn register() -> &'static str {
    "hello from register"
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
