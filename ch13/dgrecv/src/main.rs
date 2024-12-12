use std::env;
use std::io::{self, Error, ErrorKind};
use std::net::{UdpSocket, SocketAddr, ToSocketAddrs};
use std::process;

fn make_dgram_server_socket(port: u16) -> Result<UdpSocket, Error> {
    let socket = UdpSocket::bind(("0.0.0.0", port))?;
    Ok(socket)
}

fn get_internet_address(addr: &SocketAddr) -> (String, u16) {
    (addr.ip().to_string(), addr.port())
}

fn say_who_called(addr: &SocketAddr) {
    let (host, port) = get_internet_address(addr);
    println!("  from: {}:{}", host, port);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: dgrecv portnumber");
        process::exit(1);
    }

    let port: u16 = args[1].parse().expect("Invalid port number");
    let socket = match make_dgram_server_socket(port) {
        Ok(sock) => sock,
        Err(e) => {
            eprintln!("cannot make socket: {}", e);
            process::exit(2);
        }
    };

    let mut buf = vec![0u8; 1024];
    loop {
        let (msglen, src) = match socket.recv_from(&mut buf) {
            Ok((len, addr)) => (len, addr),
            Err(e) => {
                eprintln!("recvfrom error: {}", e);
                continue;
            }
        };

        if msglen == 0 {
            continue;
        }

        buf[msglen] = 0;
        println!("dgrecv: got a message: {}", String::from_utf8_lossy(&buf[..msglen]));
        say_who_called(&src);
    }
}
