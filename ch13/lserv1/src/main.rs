use std::net::UdpSocket;
use std::str;

const MSGLEN: usize = 128;
const SERVER_PORTNUM: u16 = 2020;
const TICKET_AVAIL: i32 = 0;
const MAXUSERS: usize = 3;
static mut TICKET_ARRAY: [i32; MAXUSERS] = [TICKET_AVAIL; MAXUSERS];
static mut NUM_TICKETS_OUT: i32 = 0;

fn setup() -> UdpSocket {
    let socket = UdpSocket::bind(format!("0.0.0.0:{}", SERVER_PORTNUM)).expect("Could not bind socket");
    unsafe {
        free_all_tickets();
    }
    socket
}

unsafe fn free_all_tickets() {
    for i in 0..MAXUSERS {
        TICKET_ARRAY[i] = TICKET_AVAIL;
    }
}

fn shut_down(socket: UdpSocket) {
    drop(socket);
}

fn handle_request(req: &str, addr: &std::net::SocketAddr, socket: &UdpSocket) {
    let response = unsafe {
        if req.starts_with("HELO") {
            do_hello(req)
        } else if req.starts_with("GBYE") {
            do_goodbye(req)
        } else {
            "FAIL invalid request".to_string()
        }
    };

    narrate("SAID:", &response, Some(addr));
    socket.send_to(response.as_bytes(), addr).expect("Failed to send response");
}

unsafe fn do_hello(msg_p: &str) -> String {
    if NUM_TICKETS_OUT >= MAXUSERS as i32 {
        return "FAIL no tickets available".to_string();
    }

    let x = TICKET_ARRAY.iter().position(|&r| r == TICKET_AVAIL).expect("Database corrupt");

    let pid: i32 = msg_p[5..].trim().parse().expect("Invalid pid");
    TICKET_ARRAY[x] = pid;
    NUM_TICKETS_OUT += 1;

    format!("TICK {}.{}", pid, x)
}

unsafe fn do_goodbye(msg_p: &str) -> String {
    let parts: Vec<&str> = msg_p[5..].trim().split('.').collect();
    let pid: i32 = parts[0].parse().expect("Invalid pid");
    let slot: usize = parts[1].parse().expect("Invalid slot");

    if TICKET_ARRAY[slot] != pid {
        narrate("Bogus ticket", msg_p, None);
        return "FAIL invalid ticket".to_string();
    }

    TICKET_ARRAY[slot] = TICKET_AVAIL;
    NUM_TICKETS_OUT -= 1;

    "THNX See ya!".to_string()
}

fn narrate(msg1: &str, msg2: &str, client: Option<&std::net::SocketAddr>) {
    match client {
        Some(addr) => eprintln!("\t\tSERVER: {} {} ({})", msg1, msg2, addr),
        None => eprintln!("\t\tSERVER: {} {}", msg1, msg2),
    }
}

fn main() {
    let socket = setup();

    loop {
        let mut buf = [0; MSGLEN];
        let (ret, addr) = socket.recv_from(&mut buf).expect("recv_from failed");
        let msg = str::from_utf8(&buf[..ret]).expect("Invalid UTF-8");

        narrate("GOT:", msg, Some(&addr));
        handle_request(msg, &addr, &socket);
    }
}
