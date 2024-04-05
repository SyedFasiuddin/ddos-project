use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
use std::process::exit;

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let _http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let response = b"HTTP/1.1 200 OK\r\n\r\n";
    stream.write_all(response).unwrap();
}

fn main() {
    let ip = "0.0.0.0";
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
            Ok(stream) => {
                match stream.peer_addr() {
                    Ok(addr) => eprintln!("INFO: got a connection from {addr}"),
                    Err(e) => eprintln!("INFO: got a connection, can't get address due to: {e}"),
                }
                handle_connection(stream);
            }
            Err(e) => {
                eprintln!("ERROR: incoming connection resulted in error: {e}");
                exit(1);
            }
        }
    }
}
