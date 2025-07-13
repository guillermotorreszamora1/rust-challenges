use std::io::{BufRead, BufReader, BufWriter, Read, Write};
//Implementation has to have several threads because if not it doesn't make sense
use std::net::{TcpListener, TcpStream};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
const STATIC_INFO: &str = "INFO {\"host\":\"0.0.0.0\",\"port\":4222,\"headers\":true,\"tls_available\":false,\"max_payload\":1048576,\"jetstream\":false}\r\n";

struct ClientData {
    subscriptions: Vec<Vec<String>>,
    stream: TcpStream,
    
}

//If we want really concurrent and fast topics, I would implement a topic tree but I won't at the beginning.
//Each client will have a topic
fn main() {
    let listener = TcpListener::bind("127.0.0.1:4222").unwrap();

    //let global_data :Arc<Mutex<Vec<(Vec<Vec<&str>>, Option<TcpStream>)>>> = Arc::new(Mutex::new(vec![(vec![vec![""]],None)]));
    let global_data: Arc<Mutex<Vec<ClientData>>> = Arc::new(Mutex::new(Vec::new()));

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let global_data = Arc::clone(&global_data);
        thread::spawn(|| {
            handle_connection(stream, global_data);
        });
        println!("Connection established!");
    }
}
//each client


fn handle_connection(mut stream: TcpStream, global_data: Arc<Mutex<Vec<ClientData>>>) {
    stream.write_all(STATIC_INFO.as_ref()).unwrap();
    //let mut subscription_data :&mut(&mut Vec<Vec<&str>>, Option<TcpStream>) = &mut(&mut vec![vec![""]], Some(stream.try_clone().unwrap()));
    let mut index_in_global = 0;

    let  client_data = ClientData {
        subscriptions: vec![],
        stream: stream.try_clone().unwrap(),
    };

    let mut index_in_global;

    {
        let mut data = global_data.lock().unwrap();
        data.push(client_data);
        index_in_global = data.len() - 1;
    }

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
            if *action == "SUB" {
                if words.len() < 3{

                    let topic_unparsed = String::from(*words.get(1).unwrap());
                    let topic :Vec<String> = topic_unparsed.split(".").map(|s| s.to_string()).collect();

                    let mut data = global_data.lock().unwrap();
                    if let Some(client) = data.get_mut(index_in_global) {
                        client.subscriptions.push(topic);
                    }

                    return;
                }
                response = Some("OK\r\n");
            }
        }
        //}

        if let Some(response) = response {
            stream.write_all(response.as_bytes()).unwrap();
        }
    }
}

fn check_if_subscribed(topic_list: &Vec<Vec<String>>,  topic_message :&Vec<String>) -> bool {
    for topic in topic_list {
        for index in 0..topic.len() {
            let sub_topic = topic.get(index).unwrap();
            let sub_topic_message = topic.get(index);
            if sub_topic == "*"  && sub_topic_message.is_some() {

            } else if sub_topic == ">" {
                return true;
            } else if let Some(some_sub_topic_message) = sub_topic_message && sub_topic== some_sub_topic_message {
                return true;
            } else {
                false;
            }
        }
    }
    true
}