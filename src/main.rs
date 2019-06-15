mod request;
mod response;
#[macro_use]
mod views;
mod database;
mod router;

use std::io::{BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

use mysql::Pool;

use database::mysql_connection;
use request::Request;
use response::Response;
use router::router;

const IP_LISTENER: &str = "127.0.0.1";
const PORT_LISTENER: &str = "5432";

fn main() {
    let listener = TcpListener::bind(&format!("{}:{}", IP_LISTENER, PORT_LISTENER)).unwrap();
    let db_pool = Arc::new(mysql_connection());

    loop {
        let stream = listener.accept().unwrap().0;
        let db_pool_clone = Arc::clone(&db_pool);
        thread::spawn(|| {
            read_request(stream, db_pool_clone);
        });
    }
}

fn send_response(mut stream: TcpStream, response: &[u8]) {
    stream.write_all(response).unwrap();
}

fn read_request(stream: TcpStream, db_pool: Arc<Pool>) {
    let mut reader = BufReader::new(stream);

    let res = match Request::read_request(&mut reader) {
        Ok(req) => router(req, db_pool),
        Err(status) => {
            let mut res = Response::new();

            res.status(status);
            res
        }
    };

    send_response(reader.into_inner(), &res.send());
}
