use std::process::{Command, exit};
use std::os::unix::io::AsRawFd;
use nix::unistd::{pipe, fork, ForkResult, dup2, close};

fn oops(message: &str, code: i32) -> ! {
    eprintln!("{}", message);
    exit(code);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: pipe cmd1 cmd2");
        exit(1);
    }

    let (read_end, write_end) = match pipe() {
        Ok((read_end, write_end)) => (read_end, write_end),
        Err(_) => oops("Cannot get a pipe", 1),
    };
    unsafe {
        match fork() {
            Ok(ForkResult::Parent { .. }) => {
                // Parent process: executes cmd2
                close(write_end).expect("close failed");
                if dup2(read_end.as_raw_fd(), 0).is_err() {
                    oops("could not redirect stdin", 3);
                }
                close(read_end).expect("close failed");
                Command::new(&args[2])
                    .status()
                    .unwrap_or_else(|_| oops(&args[2], 4));
            },
            Ok(ForkResult::Child) => {
                // Child process: executes cmd1
                close(read_end).expect("close failed");
                if dup2(write_end.as_raw_fd(), 1).is_err() {
                    oops("could not redirect stdout", 4);
                }
                close(write_end).expect("close failed");
                Command::new(&args[1])
                    .status()
                    .unwrap_or_else(|_| oops(&args[1], 5));
            },
            Err(_) => oops("Cannot fork", 2),
        }
    }

}
