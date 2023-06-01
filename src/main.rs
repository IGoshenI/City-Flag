#[macro_use] extern crate rocket;
use rocket::{post, response::content, routes, serde::{Deserialize, Serialize, json::Json}, fs::{FileServer, NamedFile}, State};


#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
struct AreaData {
    area_name: String,
}

struct Status{
    current_area: String,
    current_flag: String
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
fn endpoint(data: Json<AreaData>, state: &State<Status>) -> content::RawHtml<String> {
    println!("Received area name: {}", data.area_name);
    if data.area_name == state.current_area{
        content::RawHtml("working".to_string())
    }
    else{
        content::RawHtml("not this area!".to_string())
    }
}

#[launch]
fn rocket() -> _ {
    let status = Status{current_area: "Tel Aviv".to_string(), current_flag: "static/Tel-Aviv-Flag.jpg".to_string()};
    rocket::build()
        .mount("/", routes![city_flag_handler, get_image, endpoint])
        .mount("/static", FileServer::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")))
        .manage(status)
}