use nix::unistd::{fork, getpid, ForkResult};
use std::process;

fn main() {
    println!("my pid is {}", getpid());

    match unsafe { fork() } {
        Ok(ForkResult::Parent { .. }) | Ok(ForkResult::Child) => {}
        Err(err) => {
            eprintln!("Fork failed: {}", err);
            process::exit(1);
        }
    }

    match unsafe { fork() } {
        Ok(ForkResult::Parent { .. }) | Ok(ForkResult::Child) => {}
        Err(err) => {
            eprintln!("Fork failed: {}", err);
            process::exit(1);
        }
    }

    match unsafe { fork() } {
        Ok(ForkResult::Parent { .. }) | Ok(ForkResult::Child) => {}
        Err(err) => {
            eprintln!("Fork failed: {}", err);
            process::exit(1);
        }
    }

    println!("my pid is {}", getpid());
}
