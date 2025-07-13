use std::io::{BufRead, BufReader, BufWriter, Read, Write};
//Implementation has to have several threads because if not it doesn't make sense
use std::net::{TcpListener, TcpStream};
use std::thread;
const STATIC_INFO: &str = "INFO {\"host\":\"0.0.0.0\",\"port\":4222,\"headers\":true,\"tls_available\":false,\"max_payload\":1048576,\"jetstream\":false}\r\n";

//If we want really concurrent and fast topics, I would implement a topic tree but I won't
fn main() {
    let listener = TcpListener::bind("127.0.0.1:4222").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(|| {
            handle_connection(stream);
        });
        println!("Connection established!");
    }
}

fn handle_connection(mut stream: TcpStream) {
    stream.write_all(STATIC_INFO.as_ref()).unwrap();

    let mut buf_reader = BufReader::new(stream.try_clone().unwrap());
    loop {
        let mut response :Option<&str>= None;
        let mut buffer: String = String::from("");
        let _ = buf_reader.read_line(&mut buffer);
        buffer = buffer.replace("\r\n","");
        let words: Vec<_> = buffer.split(" ").collect();
        if let Some(action) = words.get(0) {
            if *action == "CONNECT" {
                response = Some("OK\r\n");
            }
            if *action == "PING" {
                response = Some("PONG\r\n");
            }
        }
        //}

        if let Some(response) = response {
            stream.write_all(response.as_bytes()).unwrap();
        }
    }
}