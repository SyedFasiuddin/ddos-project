use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
use std::process::exit;

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
            Ok(n) => buffer[0..n].iter().cloned().collect(),
            Err(e) => {
                eprintln!("ERROR: cannot read from {peer_addr} due to: {e}");
                return;
            }
        };

        let http_request = String::from_utf8(bytes).expect("ERROR: {peer_addr} violates HTTP spec");
        str_buf.push_str(&http_request);
        let http_request: Vec<_> = str_buf.lines().collect();

        for line in http_request {
            if line == "" {
                // Got a complete HTTP request
                break 'outer;
            }
        }
    }

    let response: String;
    match &str_buf.lines().nth(0).unwrap().split(' ').collect::<Vec<_>>()[..] {
        ["GET", "/test", "HTTP/1.1"] => {
            eprintln!("INFO: GET request for route /test from {peer_addr}");
            response = "HTTP/1.1 200 OK\r\n\r\nHello World\n".to_string();
        },
        _ => unimplemented!(),
    }

    match stream.write_all(response.as_bytes()) {
        Ok(()) => eprintln!("INFO: sent response to {peer_addr} successfully, closing connection"),
        Err(e) => eprintln!("ERROR: sending response to {peer_addr} due to: {e}"),
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
