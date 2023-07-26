#[macro_use]
extern crate rocket;

// #[macro_use]
extern crate diesel;

use reps::DbCon;

mod reps;

#[launch]
fn launch() -> _ {
    rocket::build().attach(DbCon::fairing())
}
