use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread::{self, Thread},
};
use httpserver::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    let thread_pool = ThreadPool::new(10);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread_pool.execute(|| {
            handle_connection(stream);
            println!("Connection established!");
        });
    }

    fn handle_connection(mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);

        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        let request_line = http_request.get(0).unwrap();

        let (status_line, file_name) = match &request_line[..] {
            "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
            _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
        };
        let contents = std::fs::read_to_string(file_name).unwrap();
        let length = contents.len();
        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
        stream.write_all(response.as_bytes()).unwrap();
        // let response = "HTTP/1.1 200 OK\r\n\r\n";

        println!("Request: {:#?}", http_request);
    }
}
