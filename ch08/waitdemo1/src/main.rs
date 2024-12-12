use nix::unistd::{fork, ForkResult, getpid};
use nix::sys::wait::wait;
use std::{thread::sleep, time::Duration, process::exit};
use nix::libc;
const DELAY: u64 = 2;

fn main() {
    println!("before: mypid is {}", getpid());

    match unsafe { fork() } {
        Ok(ForkResult::Child) => {
            child_code(DELAY);
        }
        Ok(ForkResult::Parent { child }) => {
            parent_code(child.into());
        }
        Err(err) => {
            eprintln!("Fork failed: {}", err);
            exit(1);
        }
    }
}

fn child_code(delay: u64) {
    println!("child {} here. will sleep for {} seconds", getpid(), delay);
    sleep(Duration::from_secs(delay));
    println!("child done. about to exit");
    exit(17);
}

fn parent_code(childpid: libc::pid_t) {
    match wait() {
        Ok(status) => {
            println!("done waiting for {}. Wait returned: {:?}", childpid, status);
        }
        Err(err) => {
            eprintln!("Wait failed: {}", err);
            exit(1);
        }
    }
}
