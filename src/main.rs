use rocket::response::content;
//use rocket::form::Form;
//use rocket::FromForm;
use rocket::fs::FileServer;
use rocket::fs::NamedFile;

#[macro_use] extern crate rocket;

/*
#[derive(FromForm)]
struct LoginForm {
    username: String,
}
*/

#[get("/")]
fn city_flag_handler() -> content::RawHtml<String> {
    let html_code = include_str!("city_flag.html").to_string();
    content::RawHtml(html_code)
}

#[get("/israel-map.jpg")]
async fn get_image() -> Option<NamedFile> {
    match NamedFile::open("static/israel-map.jpg").await {
        Ok(named_file) => Some(named_file),
        Err(_) => None,
    }
}

/* 
#[post("/login", data = "<form>")]
fn login_handler(form: Form<LoginForm>) -> content::RawHtml<String> {
    let username = form.username.clone();
    let response = format!("Hello {}!", username);
    content::RawHtml(response)
}
*/

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![city_flag_handler, get_image])
        .mount("/static", FileServer::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")))
}