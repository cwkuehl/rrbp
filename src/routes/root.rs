use rocket::{fs::NamedFile, http::Status, response::{Redirect}};
use crate::auth::User;
use crate::reps::DbCon;
use rep::models::Benutzer;
use rocket::serde::json::Json;

// #[get("/")]
// pub async fn get_json() -> status::Custom<content::RawJson<&'static str>> {
//      status::Custom(Status::ImATeapot, content::RawJson("{ \"hi\": \"world\" }"))
// }

#[get("/")]
pub async fn login() -> Redirect {
    Redirect::to("/auth/login")
}

#[get("/favicon.ico")]
pub async fn favicon() -> Option<NamedFile> {
    NamedFile::open("templates/icons/icon-32x32.ico").await.ok()
}

#[get("/error/<id>")]
pub async fn get_error(id: u32) -> Result<String, Status> {
  match id {
    3 => Ok("Super!".into()),
    _ => Err(Status::ImATeapot)
  }
}

/**
    List all requests contained in the database
    This handler acts upon requests accepting json responses
    We require there to be an authenticated user available
    Unfortunately we cannot direcrty implement Responder on `Vec<ContactRequest>`, hence we wrap in in json
*/
#[get("/ben")] //, format = "application/json")]
pub async fn list_requests(con: DbCon, _user: User) -> Json<Vec<Benutzer>> {
    let res = crate::reps::benutzer::get_all(&con).await.unwrap_or_default();
    Json(res)
}
