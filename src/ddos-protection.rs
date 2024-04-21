use std::path::PathBuf;

use rocket::get;
use rocket::http::ContentType;
use rocket::routes;

const SERVER_IP: &str = "127.0.0.1";
const SERVER_PORT: &str = "8000";

#[get("/test")]
async fn test() -> Option<(ContentType, Vec<u8>)> {
    let url = format!("http://{SERVER_IP}:{SERVER_PORT}/test");
    match reqwest::get(url).await {
        Ok(response) => {
            let bytes = response.bytes().await.unwrap();
            Some((ContentType::Plain, bytes.to_vec()))
        }
        Err(e) => {
            eprintln!("ERROR: something went wrong: {e}");
            None
        }
    }
}

#[get("/<file..>")]
async fn dist(file: PathBuf) -> Option<(ContentType, Vec<u8>)> {
    let url = format!("http://{SERVER_IP}:{SERVER_PORT}/{}", file.display());
    match reqwest::get(url).await {
        Ok(response) => {
            let headers = response.headers();
            let content_type = headers.get("Content-Type").unwrap().to_str().unwrap();
            let content_type = ContentType::parse_flexible(content_type).unwrap();
            let bytes = response.bytes().await.unwrap();

            Some((content_type, bytes.to_vec()))
        }
        Err(e) => {
            eprintln!("ERROR: something went wrong: {e}");
            None
        }
    }
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![test, dist])
        .launch()
        .await;
}
