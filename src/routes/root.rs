// use rocket_dyn_templates::Template;
// use serde::Serialize;
// use crate::reps::DbCon;

// /**
//     Renders the index page from a template.
//     The used Template does not need any context, hence we pass it an empty one.
// */
// #[get("/")]
// pub async fn index_template(con: DbCon) -> Template {
//     let labels = IndexLabel {
//         message: "Nachricht".into(),
//         email: "E-Mail".into()
//     };
//     Template::render("index", labels)
// }

use rocket::http::Status;
use rocket::response::{content, status};

#[get("/")]
pub async fn json() -> status::Custom<content::RawJson<&'static str>> {
    status::Custom(Status::ImATeapot, content::RawJson("{ \"hi\": \"world\" }"))
}