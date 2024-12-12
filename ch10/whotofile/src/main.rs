use std::fs::File;
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::process::{exit, Command};
use nix::unistd::{fork, ForkResult, close, dup2};
use nix::sys::wait::wait;

fn main() {
    println!("About to run who into a file");
    unsafe {
        match fork() {
            Ok(ForkResult::Parent { .. }) => {
                // parent waits then reports
                wait().expect("waitpid failed");
                println!("Done running who. Results in userlist");
            },
            Ok(ForkResult::Child) => {
                // child does the work
                close(1).expect("Failed to close stdout");
                let fd = File::create("userlist").expect("Could not create file");
                dup2(fd.as_raw_fd(), 1).expect("Could not duplicate fd to 1");

                Command::new("who")
                    .stdout(unsafe { File::from_raw_fd(1) })
                    .status()
                    .expect("Failed to execute 'who'");
                exit(0);
            },
            Err(_) => {
                eprintln!("Fork failed");
                exit(1);
            },
        }
    }
}
