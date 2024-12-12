use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream};
use std::process::{Command};
use std::os::unix::io::{FromRawFd, AsRawFd};
use nix::unistd::{fork, ForkResult, dup2, close};
use regex::Regex;

const HOSTLEN: usize = 256;
const BACKLOG: i32 = 1;

fn oops(message: &str, code: i32) -> ! {
    eprintln!("{}", message);
    std::process::exit(code);
}

fn make_server_socket(portnum: u16) -> io::Result<TcpListener> {
    TcpListener::bind(format!("0.0.0.0:{}", portnum))
}

fn connect_to_server(host: &str, portnum: u16) -> io::Result<TcpStream> {
    let addr = format!("{}:{}", host, portnum);
    TcpStream::connect(addr)
}

fn read_til_crnl(reader: &mut BufReader<TcpStream>) {
    let mut buf = String::new();
    while reader.read_line(&mut buf).unwrap() > 0 && buf != "\r\n" {
        buf.clear();
    }
}

fn process_rq(request: &str, fd: i32) {
    let mut cmd = String::new();
    let mut arg = String::from("./");

    if let Ok(ForkResult::Child) = unsafe { fork() } {
        return;
    }

    let re = Regex::new(r"^(\S+)\s+(\S+)").unwrap();
    if let Some(caps) = re.captures(request) {
        cmd = caps[1].to_string();
        arg = format!("./{}", &caps[2]);
    } else {
        return;
    }

    if cmd != "GET" {
        cannot_do(fd);
    } else if not_exist(&arg) {
        do_404(&arg, fd);
    } else if isadir(&arg) {
        do_ls(&arg, fd);
    } else if ends_in_cgi(&arg) {
        do_exec(&arg, fd);
    } else {
        do_cat(&arg, fd);
    }
}

fn cannot_do(fd: i32) {
    let mut stream = unsafe { BufWriter::new(File::from_raw_fd(fd)) };
    writeln!(stream, "HTTP/1.0 501 Not Implemented\r\nContent-type: text/plain\r\n\r\nThat command is not yet implemented\r\n").unwrap();
}

fn do_404(item: &str, fd: i32) {
    let mut stream = unsafe { BufWriter::new(File::from_raw_fd(fd)) };
    writeln!(stream, "HTTP/1.0 404 Not Found\r\nContent-type: text/plain\r\n\r\nThe item you requested: {} is not found\r\n", item).unwrap();
}

fn isadir(f: &str) -> bool {
    std::fs::metadata(f).map(|m| m.is_dir()).unwrap_or(false)
}

fn not_exist(f: &str) -> bool {
    !std::fs::metadata(f).is_ok()
}

fn do_ls(dir: &str, fd: i32) {
    let mut stream = unsafe { BufWriter::new(File::from_raw_fd(fd)) };
    writeln!(stream, "HTTP/1.0 200 OK\r\nContent-type: text/plain\r\n\r\n").unwrap();
    stream.flush().unwrap();

    dup2(fd, 1).unwrap();
    dup2(fd, 2).unwrap();
    close(fd).unwrap();
    Command::new("ls")
        .arg("-l")
        .arg(dir)
        .spawn()
        .unwrap_or_else(|_| oops("Cannot run ls", 5));
}

fn ends_in_cgi(f: &str) -> bool {
    f.ends_with(".cgi")
}

fn do_exec(prog: &str, fd: i32) {
    let mut stream = unsafe { BufWriter::new(File::from_raw_fd(fd)) };
    writeln!(stream, "HTTP/1.0 200 OK\r\n").unwrap();
    stream.flush().unwrap();

    dup2(fd, 1).unwrap();
    dup2(fd, 2).unwrap();
    close(fd).unwrap();
    Command::new(prog)
        .spawn()
        .unwrap_or_else(|_| oops("Cannot run program", 5));
}

fn do_cat(f: &str, fd: i32) {
    let ext = file_type(f);
    let content_type = match ext.as_str() {
        "html" => "text/html",
        "gif" => "image/gif",
        "jpg" | "jpeg" => "image/jpeg",
        _ => "text/plain",
    };

    let mut fpsock = unsafe { BufWriter::new(File::from_raw_fd(fd)) };
    let mut fpfile = File::open(f).unwrap();

    writeln!(fpsock, "HTTP/1.0 200 OK\r\nContent-type: {}\r\n\r\n", content_type).unwrap();
    io::copy(&mut fpfile, &mut fpsock).unwrap();
}

fn file_type(f: &str) -> String {
    f.split('.').last().unwrap_or("").to_string()
}

fn main() {
    let port: u16 = 13000;
    let listener = make_server_socket(port).unwrap_or_else(|_| oops("Failed to create server socket", 1));

    println!("Server listening on port {}", port);

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let fd = stream.as_raw_fd();
                let mut reader = BufReader::new(stream);

                let mut request = String::new();
                reader.read_line(&mut request).unwrap();
                println!("got a call: request = {}", request.trim());

                read_til_crnl(&mut reader);
                process_rq(&request, fd);
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}
