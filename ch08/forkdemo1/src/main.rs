use nix::unistd::{fork, ForkResult, getpid};
use std::thread::sleep;
use std::time::Duration;
use std::process;

fn main() {
    let mypid = getpid();
    println!("Before: my pid is {}", mypid);

    match unsafe { fork() } {
        Ok(ForkResult::Parent { child }) => {
            sleep(Duration::from_secs(1));
            println!("After: my pid is {}, fork() said {}", getpid(), child);
        }
        Ok(ForkResult::Child) => {
            sleep(Duration::from_secs(1));
            println!("After: my pid is {}, fork() said 0", getpid());
        }
        Err(err) => {
            println!("Fork failed: {}", err);
            process::exit(1);
        }
    }
}
