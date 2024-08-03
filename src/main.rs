use chrono::Local;
use serde::Serialize;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use web_server::*;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

#[derive(Serialize)]
struct TimeResponse {
    time: String,
}

/// Handle the connection by reading the request and sending an appropriate response.
///
/// The response can be a success with `hello.html`, a page that shows the time in the json format or not found with `404.html`.
fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // Read the request line to determine the response
    let (status_line, contents, content_type) = match &request_line[..] {
        "GET / HTTP/1.1" => (
            "HTTP/1.1 200 OK",
            fs::read_to_string("hello.html").unwrap(),
            "text/html",
        ),
        "GET /form HTTP/1.1" => (
            "HTTP/1.1 200 OK",
            fs::read_to_string("form.html").unwrap(),
            "text/html",
        ),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            (
                "HTTP/1.1 200 OK",
                fs::read_to_string("hello.html").unwrap(),
                "text/html",
            )
        }
        "GET /time HTTP/1.1" => {
            let time = Local::now().to_rfc2822();
            let response = TimeResponse { time };
            let contents = serde_json::to_string(&response).unwrap();
            log_to_file(&contents);
            ("HTTP/1.1 200 OK", contents, "application/json")
        }
        _ => (
            "HTTP/1.1 404 NOT FOUND",
            fs::read_to_string("404.html").unwrap(),
            "text/html",
        ),
    };

    let length = contents.len();
    let response = format!(
        "{status_line}\r\nContent-Type: {content_type}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );

    stream.write_all(response.as_bytes()).unwrap();
}
