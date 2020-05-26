use rocket::Route;

use const_concat::concat;
pub const BASE_PATH: &'static str = "/auth";
const LOGIN_URL = BASE_PATH.;
#[get(LOGIN_URL)]
pub fn login() -> &'static str{
  "hello from auth login"
}
#[get("/auth/register")]
pub fn register() -> &'static str {
  "hello from register"
}
#[get("/auth/logout")]
pub fn logout() -> &'static str {
  "ok.. logging out now."
}
#[get("/auth/session")]
pub fn get_auth_session() -> &'static str {
  "here is your auth session: "
}
#[post("/auth/session")]
pub fn start_auth_session() -> &'static str {
  "ok.. starting auth session"
}
pub fn get_routes() -> Vec<Route> {
  routes![login,register,logout, get_auth_session, start_auth_session]
}