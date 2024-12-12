use std::net::{UdpSocket, SocketAddr, Ipv4Addr};
use std::str;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;
use std::ffi::CString;

const MSGLEN: usize = 128;
const SERVER_PORTNUM: u16 = 2020;
const HOSTLEN: usize = 512;

static mut HAVE_TICKET: bool = false;
static mut TICKET_BUF: [u8; MSGLEN] = [0; MSGLEN];
static mut PID: i32 = -1;
static mut SD: Option<UdpSocket> = None;
static mut SERV_ADDR: Option<SocketAddr> = None;

fn setup() {
    unsafe {
        PID = std::process::id() as i32;
        SD = Some(UdpSocket::bind("0.0.0.0:0").expect("Cannot create socket"));
        let hostname = "localhost"; // Assuming server is on localhost
        SERV_ADDR = make_ipv4_address(hostname, SERVER_PORTNUM).ok();
    }
}

fn make_ipv4_address(hostname: &str, port: u16) -> Result<SocketAddr, &'static str> {
    use std::net::ToSocketAddrs;

    let mut addresses = format!("{}:{}", hostname, port)
        .to_socket_addrs()
        .map_err(|_| "Failed to resolve address")?;

    addresses
        .find(|addr| matches!(addr, SocketAddr::V4(_)))
        .ok_or("No IPv4 addresses found")
}

fn narrate(msg1: &str, msg2: &str) {
    unsafe {
        eprintln!("CLIENT [{}]: {} {}", PID, msg1, msg2);
    }
}

fn syserr(msg1: &str) {
    unsafe {
        eprintln!("CLIENT [{}]: {}", PID, msg1);
        std::process::exit(1);
    }
}

fn do_transaction(msg: &str) -> Option<String> {
    let mut buf = [0; MSGLEN];
    unsafe {
        if let Some(ref sd) = SD {
            if let Some(ref serv_addr) = SERV_ADDR {
                match sd.send_to(msg.as_bytes(), serv_addr) {
                    Ok(_) => (),
                    Err(e) => {
                        syserr(&format!("sendto: {}", e));
                        return None;
                    }
                }
                let (ret, _addr) = match sd.recv_from(&mut buf) {
                    Ok(res) => res,
                    Err(e) => {
                        syserr(&format!("recvfrom: {}", e));
                        return None;
                    }
                };
                let msg_str = str::from_utf8(&buf[..ret]).expect("Invalid UTF-8");
                return Some(msg_str.trim().to_string());
            }
        }
        None
    }
}

fn get_ticket() -> i32 {
    if unsafe { HAVE_TICKET } {
        return 0;
    }
    let msg = format!("HELO {}", unsafe { PID });
    if let Some(response) = do_transaction(&msg) {
        if response.starts_with("TICK") {
            unsafe {
                TICKET_BUF = [0; MSGLEN];
                let response_bytes = response.as_bytes();
                TICKET_BUF[..response_bytes.len()].copy_from_slice(response_bytes);
                HAVE_TICKET = true;
            }
            narrate("got ticket", &response[5..]);
            return 0;
        } else if response.starts_with("FAIL") {
            narrate("Could not get ticket", &response[5..]);
        } else {
            narrate("Unknown message:", &response);
        }
    }
    -1
}

fn release_ticket() -> i32 {
    if !unsafe { HAVE_TICKET } {
        return 0;
    }
    let msg = format!("GBYE {}", str::from_utf8(&unsafe { TICKET_BUF }).unwrap_or(""));
    if let Some(response) = do_transaction(&msg) {
        if response.starts_with("THNX") {
            narrate("released ticket OK", "");
            return 0;
        } else if response.starts_with("FAIL") {
            narrate("release failed", &response[5..]);
        } else {
            narrate("Unknown message:", &response);
        }
    }
    -1
}

fn do_regular_work() {
    println!("SuperSleep version 1.0 Running - Licensed Software");
    sleep(Duration::from_secs(10));
}

fn main() {
    setup();
    if get_ticket() != 0 {
        exit(0);
    }
    do_regular_work();
    release_ticket();
    unsafe {
        if let Some(sd) = SD.take() {
            drop(sd);
        }
    }
}
