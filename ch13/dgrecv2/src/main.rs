use std::env;
use std::io::{self, Error, ErrorKind};
use std::net::{UdpSocket, SocketAddr, ToSocketAddrs};
use std::process;

fn make_dgram_server_socket(port: u16) -> Result<UdpSocket, io::Error> {
    let socket = UdpSocket::bind(("0.0.0.0", port))?;
    Ok(socket)
}

fn make_internet_address(hostname: &str, port: u16) -> Result<SocketAddr, io::Error> {
    let addr = format!("{}:{}", hostname, port);
    addr.to_socket_addrs()?.next().ok_or_else(|| Error::new(ErrorKind::Other, "Invalid address"))
}

fn get_internet_address(addr: &SocketAddr) -> (String, u16) {
    (addr.ip().to_string(), addr.port())
}

fn say_who_called(addr: &SocketAddr) {
    let (host, port) = get_internet_address(addr);
    println!("  from: {}:{}", host, port);
}

fn reply_to_sender(socket: &UdpSocket, msg: &str, addr: &SocketAddr) -> Result<(), io::Error> {
    let reply = format!("Thanks for your {} char message\n", msg.len());
    socket.send_to(reply.as_bytes(), addr)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: dgrecv portnumber");
        process::exit(1);
    }

    let port: u16 = args[1].parse().expect("Invalid port number");
    let socket = make_dgram_server_socket(port)?;

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
        if let Err(e) = reply_to_sender(&socket, &String::from_utf8_lossy(&buf[..msglen]), &src) {
            eprintln!("reply_to_sender error: {}", e);
        }
    }
}
