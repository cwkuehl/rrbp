use rocket::http::Status;
use rocket::response::{content, status};

#[get("/")]
pub async fn get_json() -> status::Custom<content::RawJson<&'static str>> {
    status::Custom(Status::ImATeapot, content::RawJson("{ \"hi\": \"world\" }"))
}

#[get("/error/<id>")]
pub async fn get_error(id: u32) -> Result<String, Status> {
  match id {
    3 => Ok("Super!".into()),
    _ => Err(Status::ImATeapot)
  }
}

