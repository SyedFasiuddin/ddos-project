mod server;
use crate::server::Server;

use std::net::Ipv4Addr;
use std::net::SocketAddr;

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "public/"]
struct Assets;

fn route_test() -> Vec<u8> {
    format!(
        "\
        HTTP/1.1 200 OK\r\n\
        Content-Type: text/plain\r\n\
        Content-Length: 12\r\n\
        \r\n\
        Hello World\n\
        "
    )
    .into()
}

fn route_dist(filename: &str) -> Vec<u8> {
    let filename = if filename == "/" {
        "/index.html"
    } else {
        filename
    };

    // [1..] to remove the `/` in front
    match Assets::get(&filename[1..]) {
        Some(asset) => {
            let data = asset.data.to_vec();
            let content_type = if filename.ends_with(".css") {
                "text/css"
            } else if filename.ends_with(".png") {
                "image/png"
            } else if filename.ends_with(".html") {
                "text/html"
            } else {
                ""
            };
            let mut response: Vec<u8> = format!(
                "\
                HTTP/1.1 200 OK\r\n\
                Content-Type: {}\r\n\
                Content-Length: {}\r\n\
                \r\n\
                ",
                content_type,
                data.len().to_string().as_str()
            )
            .into();

            response.extend_from_slice(&data);
            response
        }
        None => "HTTP/1.1 404 Not Found".into(),
    }
}

fn main() {
    let server = if cfg!(target_os = "linux") {
        Server::new(Ipv4Addr::new(0, 0, 0, 0), 8000)
    } else {
        Server::new(Ipv4Addr::new(127, 0, 0, 1), 8000)
    };

    server.handle_connections(
        &|request_line: &str, peer_addr: SocketAddr| match request_line
            .split(' ')
            .collect::<Vec<_>>()[..]
        {
            ["GET", "/test", "HTTP/1.1"] => {
                eprintln!("INFO: GET request for route /test from {peer_addr}");
                route_test()
            }
            ["GET", filename, "HTTP/1.1"] => {
                eprintln!("INFO: GET request for route {filename} from {peer_addr}");
                route_dist(filename)
            }
            _ => unimplemented!(),
        },
    );
}
