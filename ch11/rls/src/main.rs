use std::env;
use std::io::{self, Write, Read};
use std::net::{TcpStream, ToSocketAddrs};
use std::process::exit;

const PORTNUM: u16 = 15000;

fn oops(message: &str) -> ! {
    eprintln!("{}", message);
    exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: rls hostname directory");
        exit(1);
    }

    let hostname = &args[1];
    let directory = &args[2];

    // Step 1: Get a socket
    let address = format!("{}:{}", hostname, PORTNUM);
    let mut stream = TcpStream::connect(address.to_socket_addrs().unwrap().next().unwrap())
        .unwrap_or_else(|_| oops("Cannot connect to server"));

    // Step 2: Send directory name
    stream.write_all(directory.as_bytes()).unwrap_or_else(|_| oops("write"));
    stream.write_all(b"\n").unwrap_or_else(|_| oops("write"));

    // Step 3: Read back results
    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => {
                io::stdout().write_all(&buffer[..n]).unwrap_or_else(|_| oops("write"));
            }
            Err(_) => oops("read"),
        }
    }
}
