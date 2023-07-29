#[macro_use]
extern crate rocket;

// #[macro_use]
extern crate diesel;

//#[macro_use]
//extern crate rocket_dyn_templates;

use reps::DbCon;
use rocket_dyn_templates::handlebars::handlebars_helper;
use rocket_dyn_templates::Template;

mod catchers;
mod reps;
mod routes;

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
