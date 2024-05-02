use std::{fmt::format, fs::read_to_string, io::{ BufRead, BufReader, Write}, net::{TcpListener, TcpStream}};

fn main() {

   let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
   for stream in listener.incoming() {
    let stream = stream.unwrap();
     handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream){
    let buf_reader = BufReader::new(&mut stream);
    let http_request = buf_reader.lines().next().unwrap().unwrap();

    if http_request == "GET / HTTP/1.1"{
        
    let status_line = "HTTP/1.1 200 OK";
    let content = read_to_string("index.html").unwrap();
    let response = format!("{status_line}\r\n\r\n\r\n{content}");
    stream.write_all(response.as_bytes()).unwrap();
    } else {
              
    let status_line = "HTTP/1.1 404 NOT FOUND";
    let content = read_to_string("404.html").unwrap();
    let response = format!("{status_line}\r\n\r\n\r\n{content}");
    stream.write_all(response.as_bytes()).unwrap();
    }
}
