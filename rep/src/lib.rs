//pub mod models;
//pub mod models_ext;
//mod res;
//pub mod revision;
mod reps;

//#[macro_use]
//extern crate diesel;

#[allow(non_snake_case)]
pub mod schema;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
