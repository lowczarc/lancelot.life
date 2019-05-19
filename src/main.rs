mod request;
mod response;
mod router;

use std::io::{Write, BufReader};
use std::net::{TcpListener, TcpStream};
use std::thread;

use request::Request;
use response::Response;
use router::router;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:5432").unwrap();
    
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

    let req_reader = Request::read_request(&mut reader);

    let res = if let Ok(req) = req_reader {
        router(req)
    } else {
        let status = if let Err(status) = req_reader {
            status
        } else {
            response::HttpStatus::InternalServerError
        };

        let mut res = Response::new();

        res.status(status);
        res
    };

    send_response(reader.into_inner(), &res.send());
}

