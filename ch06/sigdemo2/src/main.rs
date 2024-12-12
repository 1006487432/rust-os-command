use std::io::{self, Write};
use std::process::{exit, Command};
use std::thread::sleep;
use std::time::Duration;
use libc::{signal, SIG_IGN, SIGINT};

extern "C" fn ignore_signal(_: libc::c_int) {
    // This function intentionally left empty
}

fn main() {
    unsafe {
        signal(SIGINT, SIG_IGN);
    }

    println!("you can't stop me!");

    loop {
        sleep(Duration::from_secs(1));
        println!("haha");
    }
}
