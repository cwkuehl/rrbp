use rocket::{fs::NamedFile, http::Status, response::{Redirect}};
use crate::auth::User;
use crate::reps::DbCon;
use rep::models::Benutzer;
use rocket::serde::json::Json;

pub struct CachedFile(NamedFile);

impl<'r> rocket::response::Responder<'r,'r> for CachedFile {
    fn respond_to(self, req: &rocket::Request) -> rocket::response::Result<'r> {
        rocket::response::Response::build_from(self.0.respond_to(req)?)
            .raw_header("Cache-control", "max-age=86400") //  24h (24*60*60)
            .ok()
        //  Last-Modified
        // If-Modified-Since
        // ETag
        // If-Match
        // Content-Disposition
        // Cache-Control
        // Content-Range, If-Range, Range
        // Fallback Content-Type
    }
}

// #[get("/")]
// pub async fn get_json() -> status::Custom<content::RawJson<&'static str>> {
//      status::Custom(Status::ImATeapot, content::RawJson("{ \"hi\": \"world\" }"))
// }

#[get("/")]
pub async fn login() -> Redirect {
    Redirect::to("/auth/login")
}

#[get("/favicon.ico")]
pub async fn favicon() -> Option<CachedFile> {
    NamedFile::open("templates/res/icons/icon-32x32.ico").await.ok().map(|nf| CachedFile(nf))
}

// #[get("/rrbp.css")]
// pub async fn rrbp_css() -> Option<NamedFile> {
//     NamedFile::open("templates/res/css/rrbp.css").await.ok()
// }

#[get("/rrbp.css")]
pub async fn rrbp_css() -> Option<CachedFile> {
    NamedFile::open("templates/res/css/rrbp.css").await.ok().map(|nf| CachedFile(nf))
}

#[get("/bulma-dl.css")]
pub async fn bulma_dl_css() -> Option<CachedFile> {
    NamedFile::open("templates/res/css/bulma-dl.css").await.ok().map(|nf| CachedFile(nf))
}

#[get("/bulma-light.css")]
pub async fn bulma_light_css() -> Option<CachedFile> {
    // NamedFile::open("templates/res/css/bulma-light.css").await.ok()
    NamedFile::open("templates/res/css/bulma-light.css").await.ok().map(|nf| CachedFile(nf))
}

#[get("/bulma-dark.css")]
pub async fn bulma_dark_css() -> Option<CachedFile> {
    NamedFile::open("templates/res/css/bulma-dark.css").await.ok().map(|nf| CachedFile(nf))
}

#[get("/jquery-3.7.1.min.js")]
pub async fn jquery_js() -> Option<CachedFile> {
    NamedFile::open("templates/res/scripts/jquery-3.7.1.min.js").await.ok().map(|nf| CachedFile(nf))
}

#[get("/image.png")]
pub async fn image_png() -> Option<CachedFile> {
    NamedFile::open("templates/res/icons/image.png").await.ok().map(|nf| CachedFile(nf))
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
