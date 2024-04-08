use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
use std::process::exit;

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "public/"]
struct Assets;

fn handle_connection(mut stream: TcpStream) {
    let peer_addr = match stream.peer_addr() {
        Ok(addr) => {
            eprintln!("INFO: got a connection from {addr}");
            addr
        }
        Err(e) => {
            eprintln!("INFO: got a connection, can't get address due to: {e}");
            "0.0.0.0:0000"
                .parse()
                .expect("ERROR: client address not parsable")
        }
    };

    let mut buf_reader = BufReader::new(&mut stream);
    let mut str_buf = String::new();

    'outer: loop {
        let mut buffer = [0; 1024];
        let bytes: Vec<_> = match buf_reader.read(&mut buffer) {
            Ok(0) => {
                eprintln!("INFO: {peer_addr} has closed the connection");
                return;
            }
            Ok(n) => buffer[0..n].to_vec(),
            Err(e) => {
                eprintln!("ERROR: cannot read from {peer_addr} due to: {e}");
                return;
            }
        };

        let http_request = String::from_utf8(bytes).expect("ERROR: {peer_addr} violates HTTP spec");
        str_buf.push_str(&http_request);
        let http_request: Vec<_> = str_buf.lines().collect();

        for line in http_request {
            if line.is_empty() {
                // Got a complete HTTP request
                break 'outer;
            }
        }
    }

    let response: Vec<u8> = match &str_buf.lines().next().unwrap().split(' ').collect::<Vec<_>>()[..] {
        ["GET", "/test", "HTTP/1.1"] => {
            eprintln!("INFO: GET request for route /test from {peer_addr}");
            "HTTP/1.1 200 OK\r\n\r\nHello World\n".into()
        },
        ["GET", filename, "HTTP/1.1"] => {
            eprintln!("INFO: GET request for route {filename} from {peer_addr}");
            let filename = if filename == &"/" {
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
                    let mut response: Vec<u8> =
                        ("HTTP/1.1 200 Ok\r\n".to_owned() +
                        "Content-Type: " + content_type + "\r\n" +
                        "Content-Length: " + data.len().to_string().as_str() + "\r\n" +
                        "\r\n").into();
                    response.extend_from_slice(&data);
                    response
                },
                None => "HTTP/1.1 404 Not Found".into(),
            }
        },
        _ => unimplemented!(),
    };

    match stream.write_all(&response) {
        Ok(()) => eprintln!("INFO: sent response to {peer_addr} successfully, closing connection"),
        Err(e) => eprintln!("ERROR: sending response to {peer_addr} failed due to: {e}"),
    }
}

fn main() {
    let ip = "127.0.0.1";
    let port = "8000";
    let address = format!("{ip}:{port}");

    let listener = match TcpListener::bind(address) {
        Ok(listener) => {
            eprintln!("INFO: started listening on {ip}:{port}");
            listener
        }
        Err(e) => {
            eprintln!("ERROR: cannot create `TcpListener` due to: {e}");
            exit(1);
        }
    };

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream),
            Err(e) => {
                eprintln!("ERROR: incoming connection resulted in error: {e}");
                exit(1);
            }
        }
    }
}
