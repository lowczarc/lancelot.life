mod request;
mod response;
#[macro_use]
mod views;
mod router;

use std::io::{Write, BufReader};
use std::net::{TcpListener, TcpStream};
use std::thread;

use request::Request;
use response::Response;
use router::router;

const IP_LISTENER: &str = "0.0.0.0";
const PORT_LISTENER: &str = "5432";

fn main() {
    let listener = TcpListener::bind(&format!("{}:{}", IP_LISTENER, PORT_LISTENER)).unwrap();
    
    loop {
        let stream = listener.accept().unwrap().0;
        thread::spawn(|| {
            read_request(stream);
        });
    }
}

fn send_response(mut stream: TcpStream, response: &[u8]) {
    stream.write_all(response).unwrap();
}

fn read_request(stream: TcpStream) {
    let mut reader = BufReader::new(stream);

    let res = match Request::read_request(&mut reader) {
        Ok(req) => router(req),
        Err(status) => {
            let mut res = Response::new();

            res.status(status);
            res
        }
    };

    send_response(reader.into_inner(), &res.send());
}
