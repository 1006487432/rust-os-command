use std::env;
use std::io::{self, Error, ErrorKind};
use std::net::{UdpSocket, ToSocketAddrs};
use std::process;

fn make_dgram_client_socket() -> Result<UdpSocket, io::Error> {
    UdpSocket::bind("0.0.0.0:0")
}

fn make_internet_address(hostname: &str, port: u16) -> Result<std::net::SocketAddr, io::Error> {
    let addr = format!("{}:{}", hostname, port);
    addr.to_socket_addrs()?.next().ok_or_else(|| Error::new(ErrorKind::Other, "Invalid address"))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("usage: dgsend host port 'message'");
        process::exit(1);
    }

    let hostname = &args[1];
    let port: u16 = args[2].parse().expect("Invalid port number");
    let msg = &args[3];

    let socket = match make_dgram_client_socket() {
        Ok(sock) => sock,
        Err(e) => {
            eprintln!("cannot make socket: {}", e);
            process::exit(2);
        }
    };

    let addr = match make_internet_address(hostname, port) {
        Ok(addr) => addr,
        Err(e) => {
            eprintln!("make addr: {}", e);
            process::exit(4);
        }
    };

    if socket.send_to(msg.as_bytes(), addr).is_err() {
        eprintln!("sendto failed");
        process::exit(3);
    }
}
