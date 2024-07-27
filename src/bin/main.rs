use server::ThreadPool;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read_exact(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, file_name) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let content = fs::read_to_string(file_name).unwrap();
    let res = format!(
        "{}\r\nContent-Lenght: {}\r\n\r\n{}",
        status_line,
        content.len(),
        content,
    );

    stream.write_all(res.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listner = TcpListener::bind("127.0.0.1:7878").unwrap();
    let thread_pool = ThreadPool::new(4);
    for stream in listner.incoming() {
        let stream = stream.unwrap();
        thread_pool.execute(|| handle_connection(stream));
    }
}
