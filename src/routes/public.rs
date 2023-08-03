use rocket_dyn_templates::Template;
use serde::Serialize;
use crate::reps::DbCon;

#[derive(Serialize)]
pub struct IndexLabel {
    message: String,
    email: String
}

/**
    Renders the index page from a template.
    The used Template does not need any context, hence we pass it an empty one.
*/
#[get("/")]
pub async fn index_template(_con: DbCon) -> Template {
    let labels = IndexLabel {
        message: "Nachricht".into(),
        email: "E-Mail".into()
    };
    Template::render("index", labels)
}

