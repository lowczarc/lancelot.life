mod request;
mod response;
#[macro_use]
mod views;
mod database;
mod router;
mod template;

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
const PORT_LISTENER: &str = env!("PORT");

fn main() {
    let listener = TcpListener::bind(&format!("{}:{}", IP_LISTENER, PORT_LISTENER))
        .expect("Tcp listen failed");
    let db_pool = Arc::new(mysql_connection());

    loop {
        if let Ok(connection) = listener.accept() {
            let stream = connection.0;
            let db_pool_clone = Arc::clone(&db_pool);
            thread::spawn(|| {
                handle_request(stream, db_pool_clone);
            });
        }
    }
}

fn handle_request(stream: TcpStream, db_pool: Arc<Pool>) {
    let mut reader = BufReader::new(stream);

    let response = match Request::read_request(&mut reader) {
        Ok(request) => router(request, db_pool),
        Err(status) => {
            let mut res = Response::new();

            res.status(status);
            res
        }
    };

    let mut stream = reader.into_inner();
    stream
        .write_all(&response.into_bytes())
        .expect("Failed to write response");
}
