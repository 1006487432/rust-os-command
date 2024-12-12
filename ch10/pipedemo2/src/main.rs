use std::io::{self, Write};
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;
use nix::unistd::{pipe, fork, ForkResult, close};

const CHILD_MESS: &str = "I want a cookie\n";
const PAR_MESS: &str = "testing..\n";

fn oops(message: &str, code: i32) -> ! {
    eprintln!("{}", message);
    exit(code);
}

fn main() {
    let mut buf = [0u8; 1024];

    // get a pipe
    let (read_fd, write_fd) = match pipe() {
        Ok(fds) => fds,
        Err(_) => oops("cannot get a pipe", 1),
    };
    unsafe {
        match fork() {
            Ok(ForkResult::Parent { .. }) => {
                // parent process
                loop {
                    if nix::unistd::write(write_fd, PAR_MESS.as_bytes()).is_err() {
                        oops("write", 4);
                    }
                    sleep(Duration::from_secs(1));
                    let read_len = match nix::unistd::read(read_fd, &mut buf) {
                        Ok(len) => len,
                        Err(_) => break,
                    };
                    if io::stdout().write_all(&buf[..read_len]).is_err() {
                        break;
                    }
                }
            },
            Ok(ForkResult::Child) => {
                // child process
                loop {
                    if nix::unistd::write(write_fd, CHILD_MESS.as_bytes()).is_err() {
                        oops("write", 3);
                    }
                    sleep(Duration::from_secs(5));
                }
            },
            Err(_) => oops("cannot fork", 2),
        }
    }


    close(read_fd).ok();
    close(write_fd).ok();
}
