// fn fact(i: u128) -> u128 {
//     if i == 0 {
//         return 1;
//     } else {
//         return i * fact(i - 1);
//     }
// }
// fn _fact2(i: i128) -> i128 {
//     if i == 0 {
//         return 1;
//     } else {
//         return i * _fact2(i - 1);
//     }
// }

// fn give_length(s: &String) -> usize {
//     s.len()
// }

// fn main() {
//     let str: String = String::from("Hello, world!");
//     let len: usize = give_length(&str);
//     println!("Length of string: {}", len);

//     println!("{}", str);
//     println!("Hello, world!");
//     let res: u128 = fact(20);
//     println!("{}", res);
// }

use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    // Bind to the specified address and port
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Failed to bind");

    println!("Server running on http://127.0.0.1:7878");

    // Accept connections and spawn a thread for each
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    match stream.read(&mut buffer) {
        Ok(_) => {
            println!("Request received:\n{}", String::from_utf8_lossy(&buffer));

            let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nHello, world!";
            stream.write_all(response.as_bytes()).unwrap();
        }
        Err(e) => eprintln!("Failed to read from connection: {}", e),
    }
}
