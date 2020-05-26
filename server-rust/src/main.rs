#![feature(proc_macro_hygiene,decl_macro)]
#[macro_use] extern crate rocket;
mod controllers;
// use self::controllers;



#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite().mount("/", routes![hello]).mount(controllers::BasePath, routes![hello].append(&mut controllers::auth::get_routes())).launch();
}
