use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::process::{Command, exit, Stdio};
use std::ffi::CString;
use nix::libc::gethostname;

const PORTNUM: u16 = 15000;
const HOSTLEN: usize = 256;

fn oops(message: &str) -> ! {
    eprintln!("{}", message);
    exit(1);
}

fn sanitize(dirname: &str) -> String {
    dirname.chars()
        .filter(|c| c.is_alphanumeric() || *c == '/')
        .collect()
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = String::new();
    let mut reader = BufReader::new(&stream);
    reader.read_line(&mut buffer).expect("Failed to read directory name");
    let sanitized_dirname = sanitize(&buffer.trim());

    let command = format!("ls {}", sanitized_dirname);
    let output = Command::new("sh")
        .arg("-c")
        .arg(&command)
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to execute command");

    stream.write_all(&output.stdout).expect("Failed to write to client");
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

    let listener = TcpListener::bind(("0.0.0.0", PORTNUM)).unwrap_or_else(|_| oops("bind"));

    println!("Server running on {}:{}", host_str, PORTNUM);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}
