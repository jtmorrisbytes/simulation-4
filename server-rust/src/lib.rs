extern crate dotenv;

// use diesel::prelude::*;
use dotenv::dotenv;
use rocket_contrib::database;
use rocket_contrib::databases::diesel::PgConnection;
use std::env;
// use rocket_contrib::databases::diesel::pg::PgConnection::{establish};
#[database("postgres")]
pub struct Database(PgConnection);

pub fn establish_connection() -> PgConnection {
    use diesel::prelude::*;
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
