#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
mod controllers;
use controllers::auth;
// use self::controllers;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![hello])
        .mount(
            &(controllers::BasePath.to_string() + auth::BASE_PATH),
            routes![
                auth::login,
                auth::register,
                auth::logout,
                auth::get_auth_session,
                auth::start_auth_session
            ],
        )
        .launch();
}
