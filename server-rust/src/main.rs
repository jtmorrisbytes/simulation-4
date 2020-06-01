#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate rocket_contrib;
extern crate serde;

mod controllers;
mod errors;
mod lib;
mod models;
mod schema;
mod session;

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
            &(controllers::BASE_PATH.to_string() + auth::BASE_PATH),
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
