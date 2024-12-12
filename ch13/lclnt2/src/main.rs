use std::io::{self, Error, ErrorKind};
use std::net::{UdpSocket, SocketAddr, ToSocketAddrs};
use std::process;
use std::thread::sleep;
use std::time::Duration;

static mut PID: i32 = -1;
static mut HAVE_TICKET: bool = false;
static mut TICKET_BUF: [u8; 128] = [0; 128];

const MSGLEN: usize = 128;
const SERVER_PORTNUM: u16 = 2020;

fn setup() -> Result<UdpSocket, Error> {
    unsafe {
        PID = process::id() as i32;
    }

    let socket = UdpSocket::bind("0.0.0.0:0")?;
    let server_addr: SocketAddr = ("127.0.0.1:2020").parse().map_err(|_| Error::new(ErrorKind::Other, "Invalid server address"))?;
    socket.connect(server_addr)?;
    Ok(socket)
}

fn shut_down(socket: UdpSocket) {
    drop(socket);
}

fn get_ticket(socket: &UdpSocket) -> Result<(), Error> {
    unsafe {
        if HAVE_TICKET {
            return Ok(());
        }

        let msg = format!("HELO {}", PID);
        let response = do_transaction(socket, &msg)?;

        if response.starts_with("TICK") {
            TICKET_BUF[..response.len() - 5].copy_from_slice(&response.as_bytes()[5..]);
            HAVE_TICKET = true;
            narrate("got ticket", &response[5..]);
            Ok(())
        } else {
            narrate("Could not get ticket", &response);
            Err(Error::new(ErrorKind::Other, "Failed to get ticket"))
        }
    }
}

fn release_ticket(socket: &UdpSocket) -> Result<(), Error> {
    unsafe {
        if !HAVE_TICKET {
            return Ok(());
        }

        let msg = format!("GBYE {}", String::from_utf8_lossy(&TICKET_BUF));
        let response = do_transaction(socket, &msg)?;

        if response.starts_with("THNX") {
            narrate("released ticket OK", "");
            Ok(())
        } else {
            narrate("release failed", &response[5..]);
            Err(Error::new(ErrorKind::Other, "Failed to release ticket"))
        }
    }
}

fn do_transaction(socket: &UdpSocket, msg: &str) -> Result<String, Error> {
    let mut buf = [0; MSGLEN];
    socket.send(msg.as_bytes())?;
    let len = socket.recv(&mut buf)?;
    Ok(String::from_utf8_lossy(&buf[..len]).to_string())
}

fn narrate(msg1: &str, msg2: &str) {
    unsafe {
        eprintln!("CLIENT [{}]: {} {}", PID, msg1, msg2);
    }
}

fn validate_ticket(socket: &UdpSocket) -> Result<(), Error> {
    unsafe {
        if !HAVE_TICKET {
            return Ok(());
        }

        let msg = format!("VALD {}", String::from_utf8_lossy(&TICKET_BUF));
        let response = do_transaction(socket, &msg)?;

        narrate("Validated ticket: ", &response);

        if response.starts_with("GOOD") {
            Ok(())
        } else {
            HAVE_TICKET = false;
            Err(Error::new(ErrorKind::Other, "Invalid ticket"))
        }
    }
}

fn do_regular_work(socket: &UdpSocket) {
    println!("SuperSleep version 1.0 Running - Licensed Software");
    sleep(Duration::from_secs(15));

    if validate_ticket(socket).is_err() {
        println!("Server errors. Please try later.");
        return;
    }

    sleep(Duration::from_secs(15));
}

fn main() {
    let socket = match setup() {
        Ok(sock) => sock,
        Err(err) => {
            eprintln!("Failed to setup: {}", err);
            return;
        }
    };

    if get_ticket(&socket).is_ok() {
        do_regular_work(&socket);
        release_ticket(&socket).unwrap_or_else(|err| eprintln!("Failed to release ticket: {}", err));
    }

    shut_down(socket);
}
