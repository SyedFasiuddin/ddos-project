use std::env;
use std::net::IpAddr;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::{Duration, SystemTime};

use rocket::get;
use rocket::http::ContentType;
use rocket::routes;
use rocket::State;

const SERVER_IP: &str = "127.0.0.1";
const SERVER_PORT: &str = "8000";

#[derive(Debug, Default)]
struct ServerState {
    req: Mutex<Vec<(IpAddr, SystemTime)>>,
}

#[get("/test")]
async fn test(ip: IpAddr, state: &State<ServerState>) -> Option<(ContentType, Vec<u8>)> {
    if should_block(ip, state) {
        return None;
    }

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
async fn dist(
    file: PathBuf,
    ip: IpAddr,
    state: &State<ServerState>,
) -> Option<(ContentType, Vec<u8>)> {
    if should_block(ip, state) {
        return None;
    }

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

fn should_block(ip: IpAddr, state: &State<ServerState>) -> bool {
    let now = SystemTime::now();
    let connections = &mut *state.req.lock().unwrap();
    let time = {
        let time = env::var("DDOS_TIMEOUT_DURATION").unwrap_or("5".to_string());
        time.parse::<u64>().unwrap()
    };
    connections.retain(|c| now.duration_since(c.1).unwrap() < Duration::new(time, 0));
    let count = connections.iter().filter(|&c| c.0 == ip).count();

    let count_limit = {
        let count = env::var("DDOS_LIMIT_WITHIN_DURATION").unwrap_or("50".to_string());
        count.parse::<usize>().unwrap()
    };
    if count >= count_limit {
        println!("INFO: blocking ip: {ip}");
        return true;
    }
    connections.push((ip, now));

    false
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![test, dist])
        .manage(ServerState::default())
        .launch()
        .await;
}
