#[macro_use]
extern crate rocket;

//#[macro_use]
//extern crate diesel;

// fn main() {
//     println!("Hello, world!");
// }

#[launch]
fn launch() -> _ {
    rocket::build()
}
