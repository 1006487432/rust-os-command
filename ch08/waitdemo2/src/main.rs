use nix::unistd::{fork, ForkResult, getpid};
use nix::sys::wait::{waitpid, WaitStatus};
use std::{thread::sleep, time::Duration, process::exit};

const DELAY: u64 = 5;

fn main() {
    println!("before: my pid is {}", getpid());

    match unsafe { fork() } {
        Ok(ForkResult::Child) => {
            child_code(DELAY);
        }
        Ok(ForkResult::Parent { child }) => {
            parent_code(child);
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

fn parent_code(childpid: nix::unistd::Pid) {
    match waitpid(childpid, None) {
        Ok(WaitStatus::Exited(_, status)) => {
            let high_8 = (status >> 8) & 0xFF;
            let low_7 = status & 0x7F;
            let bit_7 = (status & 0x80) >> 7;
            println!("done waiting for {}. Wait returned: {}", childpid, status);
            println!("status: exit={}, sig={}, core={}", high_8, low_7, bit_7);
        }
        Ok(status) => {
            println!("Unexpected status: {:?}", status);
        }
        Err(err) => {
            eprintln!("Wait failed: {}", err);
            exit(1);
        }
    }
}
