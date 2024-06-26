use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::net::Ipv4Addr;
use std::net::TcpListener;
use std::net::TcpStream;
use std::net::{SocketAddr, SocketAddrV4};
use std::process::exit;

pub struct Server<'a> {
    listener: TcpListener,
    generate_response: &'a dyn Fn(&str, SocketAddr) -> Vec<u8>,
}

impl<'a> Server<'a> {
    pub fn new(ip: Ipv4Addr, port: u16) -> Self {
        Self {
            listener: match TcpListener::bind(SocketAddrV4::new(ip, port)) {
                Ok(listener) => {
                    eprintln!("INFO: started listening on {ip}:{port}");
                    listener
                }
                Err(e) => {
                    eprintln!("ERROR: cannot create `TcpListener` due to: {e}");
                    exit(1);
                }
            },
            generate_response: &Self::dumb,
        }
    }

    fn dumb(_a: &str, _b: SocketAddr) -> Vec<u8> {
        unimplemented!()
    }

    fn handle_connection(&self, mut stream: TcpStream) {
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
        let mut http_request_buffer = String::new();

        'outer: loop {
            let mut buffer = [0; 1024];
            let bytes: Vec<_> = match buf_reader.read(&mut buffer) {
                Ok(0) => {
                    eprintln!("INFO: {peer_addr} has closed the connection");
                    return;
                }
                Ok(n) => {
                    eprintln!("INFO: recieved {n} bytes from {peer_addr}");
                    buffer[0..n].to_vec()
                }
                Err(e) => {
                    eprintln!("ERROR: cannot read from {peer_addr} due to: {e}");
                    return;
                }
            };

            let http_request = match String::from_utf8(bytes) {
                Ok(str) => str,
                Err(_) => {
                    eprintln!("ERROR: {peer_addr} violates HTTP spec, closing connection");
                    return;
                }
            };
            http_request_buffer.push_str(&http_request);

            for line in http_request_buffer.lines() {
                if line.is_empty() {
                    // Got a complete HTTP request
                    break 'outer;
                }
            }
        }

        let request_line = match http_request_buffer.lines().next() {
            Some(line) => line,
            None => {
                eprintln!("ERROR: {peer_addr} violates HTTP spec, closing connection");
                return;
            }
        };

        let response = (self.generate_response)(request_line, peer_addr);
        match stream.write_all(&response) {
            Ok(()) => {
                eprintln!("INFO: sent response to {peer_addr} successfully, closing connection")
            }
            Err(e) => eprintln!("ERROR: sending response to {peer_addr} failed due to: {e}"),
        }
    }

    pub fn handle_connections(
        mut self,
        generate_response: &'a dyn Fn(&str, SocketAddr) -> Vec<u8>,
    ) {
        self.generate_response = generate_response;
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => self.handle_connection(stream),
                Err(e) => {
                    eprintln!("ERROR: incoming connection resulted in error: {e}");
                    exit(1);
                }
            }
        }
    }
}
