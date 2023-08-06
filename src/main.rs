// Framework inspired by book Rust (Marco Amann, Joachim Baumann, Marcel Koch),
// see https://github.com/dxfrontiers/rust-buch/blob/master/06_webprogrammierung
#[macro_use]
extern crate rocket;
extern crate diesel;
extern crate rep;

//#[macro_use]
//extern crate rocket_dyn_templates;

mod auth;
mod base;
mod catchers;
mod reps;
mod routes;

use base::functions;
use reps::DbCon;
use rocket_dyn_templates::handlebars::handlebars_helper;
use rocket_dyn_templates::Template;

// define a helper using helper
handlebars_helper!(include_css: |v: str| {
  let mut s = String::new();
  s.push_str("aaa ");
  s.push_str(v);
  s.push_str(" zzz");
  s
  // let mut x = v.to_string();
  // x.push_str(" abc");
  // x
});
//handlebars_helper!(nargs: |*args| args.len());
//handlebars_helper!(hbh: |*args| args[1].to_string());
// fn upper(h: &Helper, rc: &mut RenderContext) -> Result<(), RenderError> {
//     // get parameter from helper or throw an error
//     let param = h.param(0).and_then(|v| v.value().as_string()).unwrap_or("");
//     //try!(rc.writer.write(param.to_uppercase().into_bytes().as_ref()));
//     rc.writer.write(param.to_uppercase().into_bytes().as_ref());
//     Ok(())
// }
handlebars_helper!(to_lower_case: |v: str| v.to_lowercase());

#[launch]
fn launch() -> _ {
    functions::mach_nichts();
    let session_store = crate::auth::SessionStorage::new();
    rocket::build()
        .attach(DbCon::fairing())
        .attach(Template::custom(|engines| {
            engines
                .handlebars
                .register_helper("include_css", Box::new(include_css));
            engines
                .handlebars
                .register_helper("to_lower_case", Box::new(to_lower_case));
        }))
        //.attach(Template::fairing())
        .manage(session_store)
        .mount(
            "/",
            routes![
                routes::root::get_json,
                routes::root::get_error,
                routes::root::list_requests,
            ],
        )
        .mount(
            "/public/",
            routes![
                //routes::public::index_json,
                routes::public::index_template,
                //routes::public::index_form,
            ],
        )
        .mount(
            "/auth/",
            routes![
                auth::login_template,
                auth::logout_template,
                auth::login_form,
            ],
        )
        .register("/", catchers![catchers::unauthorized])
}
