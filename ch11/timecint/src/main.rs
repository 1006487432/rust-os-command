use std::env;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::process::exit;

fn oops(message: &str) -> ! {
    eprintln!("{}", message);
    exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: timeclnt hostname portnumber");
        exit(1);
    }

    let hostname = &args[1];
    let portnumber = &args[2];

    // Step 1: Get a socket
    let address = format!("{}:{}", hostname, portnumber);
    let mut stream = TcpStream::connect(&address).unwrap_or_else(|_| oops("socket"));

    // Step 2: Connect to server
    // This step is essentially combined with Step 1 in Rust

    // Step 3: Transfer data from server, then hangup
    let mut message = vec![0; 1024];
    let messlen = stream.read(&mut message).unwrap_or_else(|_| oops("read"));
    if messlen == 0 {
        oops("read");
    }

    io::stdout().write_all(&message[..messlen]).unwrap_or_else(|_| oops("write"));
    stream.shutdown(std::net::Shutdown::Both).unwrap();
}
