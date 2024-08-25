use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

use crate::thread_pool::ThreadPool;

mod thread_pool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
        // println!("Connection established! {stream:?}");
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    // println!("Request: {http_request:#?}");

    let request_route_line = match http_request.first() {
        Some(line) => line,
        None => return (),
    };

    println!("Request: {request_route_line:#?}");

    let (res_status_line, res_filename) = match &request_route_line[..] {
        "GET /hello HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /other HTTP/1.1" => ("HTTP/1.1 200 OK", "other.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(9));
            ("HTTP/1.1 200 OK", "other.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(res_filename).unwrap();
    let length = contents.len();
    let response = format!("{res_status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
