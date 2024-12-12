use nix::unistd::{fork, getpid, ForkResult};
use std::process;

fn main() {
    println!("Before: my pid is {}", getpid());

    match unsafe { fork() } {
        Ok(ForkResult::Parent { child }) => {
            println!("I am the parent. my child is {}", child);
        }
        Ok(ForkResult::Child) => {
            println!("I am the child.  my pid={}", getpid());
        }
        Err(err) => {
            eprintln!("Fork failed: {}", err);
            process::exit(1);
        }
    }
}
