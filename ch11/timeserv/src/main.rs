use std::io::{self, Write};
use std::net::{TcpListener, TcpStream};
use std::time::{SystemTime, UNIX_EPOCH};
use std::process::exit;
use std::ffi::CString;
use nix::libc::gethostname;

const PORTNUM: u16 = 13000;
const HOSTLEN: usize = 256;

fn oops(message: &str) -> ! {
    eprintln!("{}", message);
    exit(1);
}

fn handle_client(mut stream: TcpStream) {
    let now = SystemTime::now();
    let since_epoch = now.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let thetime = since_epoch.as_secs();

    let time_string = format!("The time here is .. {}\n", thetime);
    stream.write_all(time_string.as_bytes()).expect("Failed to write to client");
}

fn get_hostname() -> String {
    let mut hostname = vec![0u8; HOSTLEN];
    unsafe {
        if gethostname(hostname.as_mut_ptr() as *mut i8, HOSTLEN) != 0 {
            oops("gethostname failed");
        }
    }
    let end = hostname.iter().position(|&x| x == 0).unwrap_or(HOSTLEN);
    String::from_utf8_lossy(&hostname[..end]).to_string()
}

fn main() {
    let host_str = get_hostname();
    println!("Server running on {}:{}", host_str, PORTNUM);

    let listener = TcpListener::bind(("0.0.0.0", PORTNUM)).unwrap_or_else(|_| oops("bind"));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Wow! got a call!");
                handle_client(stream);
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}
