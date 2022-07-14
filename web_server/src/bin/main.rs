use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    listener.set_nonblocking(true).unwrap();

    let pool = ThreadPool::new(7);
    let run = Arc::new(Mutex::new(true));

    let run_ptr = Arc::clone(&run);
    let handle = thread::spawn(|| {
        handle_input(run_ptr);
    });

    let run_ptr = Arc::clone(&run);
    while *run_ptr.lock().unwrap() == true {
        let stream = match listener.accept() {
            Ok((socket, _addr)) => socket,
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => continue,
            Err(e) => panic!("encountered IO error: {}", e),
        };

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    handle.join().expect("Unable to shut down input thread");
    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    if let Err(e) = stream.read(&mut buffer) {
        if e.kind() == std::io::ErrorKind::WouldBlock {
            return;
        } else {
            panic!("encountered IO error: {}", e);
        }
    }

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).expect("Failed to read .html file");

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream
        .write(response.as_bytes())
        .expect("Failed to write to TcpStream");
    stream.flush().unwrap();
}

fn handle_input(run_ptr: Arc<Mutex<bool>>) {
    loop {
        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input line");

        let input: String = match input.trim().parse() {
            Ok(s) => s,
            Err(_) => {
                println!("Bad input");
                continue;
            }
        };

        if input == ":q" {
            let mut b = run_ptr.lock().unwrap();
            *b = false;
            break;
        }
    }
}
