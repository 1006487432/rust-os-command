use std::collections::HashMap;
use std::io::{self};
use std::net::{SocketAddr, UdpSocket};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const SERVER_PORT: u16 = 2020;
const MSGLEN: usize = 128;
const TICKET_AVAIL: i32 = 0;
const MAXUSERS: usize = 3;
const RECLAIM_INTERVAL: Duration = Duration::from_secs(5);

type Tickets = Arc<Mutex<HashMap<usize, i32>>>;

fn main() -> io::Result<()> {
    let ticket_array: Tickets = Arc::new(Mutex::new(HashMap::new()));
    for i in 0..MAXUSERS {
        ticket_array.lock().unwrap().insert(i, TICKET_AVAIL);
    }

    let socket = UdpSocket::bind(("0.0.0.0", SERVER_PORT))?;
    println!("Server listening on port {}", SERVER_PORT);

    let ticket_array_clone = Arc::clone(&ticket_array);
    thread::spawn(move || {
        loop {
            thread::sleep(RECLAIM_INTERVAL);
            ticket_reclaim(&ticket_array_clone);
        }
    });

    let mut buf = [0; MSGLEN];
    loop {
        let (amt, src) = socket.recv_from(&mut buf)?;
        let msg = String::from_utf8_lossy(&buf[..amt]).to_string();
        handle_request(&socket, &msg, &src, &ticket_array)?;
    }
}

fn handle_request(
    socket: &UdpSocket,
    req: &str,
    client: &SocketAddr,
    ticket_array: &Tickets,
) -> io::Result<()> {
    let response = if req.starts_with("HELO") {
        do_hello(req, ticket_array)
    } else if req.starts_with("GBYE") {
        do_goodbye(req, ticket_array)
    } else if req.starts_with("VALD") {
        do_validate(req, ticket_array)
    } else {
        "FAIL invalid request".to_string()
    };

    println!("SAID: {}", response);
    socket.send_to(response.as_bytes(), client)?;
    Ok(())
}

fn do_hello(req: &str, ticket_array: &Tickets) -> String {
    let pid: i32 = req[5..].trim().parse().unwrap_or(-1);
    let mut tickets = ticket_array.lock().unwrap();
    if tickets.values().filter(|&&v| v != TICKET_AVAIL).count() >= MAXUSERS {
        return "FAIL no tickets available".to_string();
    }

    if let Some((&i, _)) = tickets.iter().find(|&(_, &v)| v == TICKET_AVAIL) {
        tickets.insert(i, pid);
        return format!("TICK {}.{}", pid, i);
    }

    "FAIL database corrupt".to_string()
}

fn do_goodbye(req: &str, ticket_array: &Tickets) -> String {
    let mut tickets = ticket_array.lock().unwrap();
    let parts: Vec<&str> = req[5..].trim().split('.').collect();
    if parts.len() != 2 {
        return "FAIL invalid ticket".to_string();
    }

    let pid: i32 = parts[0].parse().unwrap_or(-1);
    let slot: usize = parts[1].parse().unwrap_or(MAXUSERS);

    if slot >= MAXUSERS || tickets.get(&slot) != Some(&pid) {
        return "FAIL invalid ticket".to_string();
    }

    tickets.insert(slot, TICKET_AVAIL);
    "THNX See ya!".to_string()
}

fn do_validate(req: &str, ticket_array: &Tickets) -> String {
    let tickets = ticket_array.lock().unwrap();
    let parts: Vec<&str> = req[5..].trim().split('.').collect();
    if parts.len() != 2 {
        return "FAIL invalid ticket".to_string();
    }

    let pid: i32 = parts[0].parse().unwrap_or(-1);
    let slot: usize = parts[1].parse().unwrap_or(MAXUSERS);

    if slot < MAXUSERS && tickets.get(&slot) == Some(&pid) {
        return "GOOD Valid ticket".to_string();
    }

    "FAIL invalid ticket".to_string()
}

fn ticket_reclaim(ticket_array: &Tickets) {
    let mut tickets = ticket_array.lock().unwrap();
    let mut to_reclaim = vec![];

    for (&slot, &pid) in tickets.iter() {
        if pid != TICKET_AVAIL {
            match nix::sys::signal::kill(nix::unistd::Pid::from_raw(pid), None) {
                Ok(_) => {}
                Err(nix::errno::Errno::ESRCH) => {
                    to_reclaim.push(slot);
                }
                _ => {}
            }
        }
    }

    for slot in to_reclaim {
        tickets.insert(slot, TICKET_AVAIL);
        println!("freeing {}", slot);
    }
}
