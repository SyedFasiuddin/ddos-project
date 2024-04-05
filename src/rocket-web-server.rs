use std::borrow::Cow;
use std::ffi::OsStr;
use std::path::PathBuf;

use rocket::get;
use rocket::http::ContentType;
use rocket::response::content::RawHtml;
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
fn index() -> Option<RawHtml<Cow<'static, [u8]>>> {
    let assets = Assets::get("index.html")?;
    Some(RawHtml(assets.data))
}

#[get("/<file..>")]
fn dist(file: PathBuf) -> Option<(ContentType, Cow<'static, [u8]>)> {
    let filename = file.display().to_string();
    let asset = Assets::get(&filename)?;
    let content_type = file
        .extension()
        .and_then(OsStr::to_str)
        .and_then(ContentType::from_extension)
        .unwrap_or(ContentType::Bytes);

    Some((content_type, asset.data))
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![hello, index, dist])
        .launch()
        .await;
}
