#[macro_use] extern crate rocket;
use rocket::{post, response::content, routes, serde::{Deserialize, Serialize, json::Json}, fs::{FileServer, NamedFile}};


#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
struct AreaData {
    area_name: String,
}

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

#[post("/api/endpoint", format = "json", data = "<data>")]
fn endpoint(data: Json<AreaData>) -> content::RawHtml<String> {
    println!("Received area name: {}", data.area_name);
    content::RawHtml("working".to_string())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![city_flag_handler, get_image, endpoint])
        .mount("/static", FileServer::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")))
}