use std::io::{self, Read, Write};
use std::process::exit;
use nix::unistd::{pipe, close};

fn main() {
    let mut buf = [0u8; 1024];

    // get a pipe
    let (read_fd, write_fd) = match pipe() {
        Ok(fds) => fds,
        Err(_) => {
            eprintln!("could not make pipe");
            exit(1);
        }
    };
    println!("Got a pipe! It is file descriptors: {{ {} {} }}", read_fd, write_fd);

    // read from stdin, write into pipe, read from pipe, print
    let stdin = io::stdin();
    let stdout = io::stdout();

    loop {
        let len = match stdin.lock().read(&mut buf) {
            Ok(len) => len,
            Err(_) => {
                eprintln!("Error reading from stdin");
                break;
            }
        };
        if len == 0 {
            break;
        }
        if nix::unistd::write(write_fd, &buf[..len]).is_err() {
            eprintln!("writing to pipe");
            break;
        }
        for i in 0..len {
            buf[i] = b'X';
        }
        let len = match nix::unistd::read(read_fd, &mut buf) {
            Ok(len) => len,
            Err(_) => {
                eprintln!("reading from pipe");
                break;
            }
        };
        if stdout.lock().write_all(&buf[..len]).is_err() {
            eprintln!("writing to stdout");
            break;
        }
    }

    close(read_fd).ok();
    close(write_fd).ok();
}
