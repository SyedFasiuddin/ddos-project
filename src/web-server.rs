use std::borrow::Cow;

use rocket::get;
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::routes;

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "public/"]
struct Assets;

#[get("/test")]
fn hello() -> &'static str {
    "Hello World\n"
}

#[get("/")]
fn index() -> (Status, (ContentType, Cow<'static, [u8]>)) {
    match Assets::get("index.html") {
        Some(embedded_file) => (Status::Ok, (ContentType::HTML, embedded_file.data)),
        None => {
            eprintln!("ERROR: unable to get `public/index.html`");
            return (
                Status::NotFound,
                (ContentType::Plain, Cow::from("404".as_bytes())),
            );
        }
    }
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![hello, index])
        .launch()
        .await;
}
