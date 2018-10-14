#[derive(FromForm)]
struct ExtId {
    ext_id: String,
}

#[get("/id?<ext_id>", format = "text/plain")]
fn resolve_external_id(ext_id: ExtId) -> &'static str {
    "Hello, world!"
}

pub fn startup() {
    rocket::ignite()
        .mount("/", routes![resolve_external_id])
        .launch();
}
