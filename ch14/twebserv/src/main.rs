use std::fs;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::path::Path;

struct ServerState {
    started: SystemTime,
    bytes_sent: u64,
    requests: u64,
}

impl ServerState {
    fn new() -> Self {
        ServerState {
            started: SystemTime::now(),
            bytes_sent: 0,
            requests: 0,
        }
    }
}

async fn handle_connection(mut stream: TcpStream, state: Arc<Mutex<ServerState>>) {
    let mut buffer = vec![0; 1024];
    stream.read(&mut buffer).await.unwrap();

    let request_line = String::from_utf8_lossy(&buffer);
    let request_line = request_line.lines().next().unwrap_or("");

    if request_line.starts_with("GET") {
        let path = request_line.split_whitespace().nth(1).unwrap_or("/");
        let path = &path[1..]; // Remove the leading "/"
        let response = if path == "status" {
            let state = state.lock().unwrap();
            let status_response = format!(
                "Server started: {:?}\nTotal requests: {}\nBytes sent out: {}",
                state.started,
                state.requests,
                state.bytes_sent
            );
            format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\n{}", status_response)
        } else if !Path::new(path).exists() {
            format!("HTTP/1.1 404 NOT FOUND\r\nContent-Type: text/plain\r\n\r\nFile Not Found")
        } else {
            let contents = fs::read_to_string(path).unwrap_or_else(|_| "Error reading file".to_string());
            let mut state = state.lock().unwrap();
            state.bytes_sent += contents.len() as u64;
            drop(state); // Release the lock before sending the response
            format!(
                "HTTP/1.1 200 OK\r\nContent-Type: {}\r\n\r\n{}",
                mime_guess::from_path(path).first_or_octet_stream(),
                contents
            )
        };
        stream.write_all(response.as_bytes()).await.unwrap();
        stream.flush().await.unwrap();

        // Increment the request counter here
        let mut state = state.lock().unwrap();
        state.requests += 1;
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <port>", args[0]);
        std::process::exit(1);
    }

    let port = args[1].parse::<u16>().unwrap();
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await.unwrap();

    let state = Arc::new(Mutex::new(ServerState::new()));

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let state = state.clone();
        tokio::spawn(async move {
            handle_connection(stream, state).await;
        });
    }
}
