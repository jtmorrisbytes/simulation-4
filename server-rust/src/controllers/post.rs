use crate::lib::Database;
use rocket::Rocket;
pub const BASE_PATH: &str = "/posts";
#[post("/<user_id>")]
pub fn create_post(conn: Database, user_id: i32) {
    use crate::schema::posts::dsl::*;
}
#[get("/<user_id>?<search>&<userposts>")]
pub fn find_post(conn: Database, user_id: i32, search: String, userposts: bool) {
    use crate::schema::posts::dsl::*;
}
fn get_post(conn: Database, user_id: i32) {
    use crate::schema::users::dsl::*;
}
fn rocket() -> Rocket {
    rocket::custom(crate::config())
        .attach(Database::fairing())
        .mount(BASE_PATH, routes![create_post, find_post])
}

#[cfg(test)]
pub mod test {
    use super::rocket;
    use crate::lib::establish_connection;
    use rocket::local::Client;
    fn client() -> Client {
        Client::new(rocket()).expect("Invalid rocket instance")
    }
    #[test]
    fn test_create_post() {
        let cli = client();
    }
}
