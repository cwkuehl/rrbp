#[macro_use]
extern crate rocket;

// #[macro_use]
extern crate diesel;

use reps::DbCon;
use rocket_dyn_templates::Template;

mod catchers;
mod reps;
mod routes;

#[launch]
fn launch() -> _ {
    rocket::build()
        .attach(DbCon::fairing())
        .attach(Template::fairing())
        .mount(
            "/public/",
            routes![
                //routes::public::index_json,
                routes::public::index_template,
                //routes::public::index_form,
            ],
        )
        .register("/", catchers![catchers::unauthorized])
}
