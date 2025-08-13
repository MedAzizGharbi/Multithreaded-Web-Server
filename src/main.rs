use multi_threaded_server::ThreadPool;
use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); //Result<T,E>
    // We will listen to connections comming on the address
    let pool = ThreadPool::new(4);
    // for stream in listener.incoming().take(2) { Hedhi lel tejrab
    for stream in listener.incoming() {
        //Traja3 iterator mta3 TcpStream
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
    println!("Shutting Down!");
}
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let (status_line, filename) = if request_line == "GET /home HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length : {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
    // let http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();
    // Ekher haja wselnelha : Sending Requests to Threads via Channels
    // Chwaya terms : Threads , Channels, Results, Closures
}
