use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::process::{exit, Command, Stdio};
use nix::unistd::{fork, pipe, dup2, close, ForkResult};
use nix::sys::wait::wait;

const READ: i32 = 0;
const WRITE: i32 = 1;

fn popen(command: &str, mode: &str) -> Option<File> {
    let (parent_end, child_end) = if mode == "r" {
        (READ, WRITE)
    } else if mode == "w" {
        (WRITE, READ)
    } else {
        return None;
    };

    let (read_fd, write_fd) = pipe().ok()?;

    match unsafe { fork() } {
        Ok(ForkResult::Parent { .. }) => {
            if child_end == WRITE {
                close(write_fd).ok()?;
                Some(unsafe { File::from_raw_fd(read_fd) })
            } else {
                close(read_fd).ok()?;
                Some(unsafe { File::from_raw_fd(write_fd) })
            }
        }
        Ok(ForkResult::Child) => {
            if parent_end == READ {
                close(read_fd).ok();
                dup2(write_fd, child_end).ok();
                close(write_fd).ok();
            } else {
                close(write_fd).ok();
                dup2(read_fd, child_end).ok();
                close(read_fd).ok();
            }
            Command::new("/bin/sh")
                .arg("-c")
                .arg(command)
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()
                .expect("Failed to execute command");
            exit(0);
        }
        Err(_) => {
            close(read_fd).ok()?;
            close(write_fd).ok()?;
            None
        }
    }
}

fn main() {
    let command = "ls";
    if let Some(file) = popen(command, "r") {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            println!("{}", line.expect("Failed to read line"));
        }
    } else {
        eprintln!("Failed to open pipe");
    }
}
