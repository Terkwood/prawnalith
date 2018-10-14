use rocket::request::{Form, LenientForm};

#[derive(FromForm)]
struct ExtId{ ext_id: String }

#[get("/id?<ext_id>")]
pub fn resolve_external_id(ext_id: Option<Form<ExtId>>) -> &'static str {
    "Hello, world!"
}