use std::io;
use std::thread::sleep;
use std::time::Duration;
use libc::{signal, SIGINT};

extern "C" fn handle_signal(_: libc::c_int) {
    println!("OUCH!");
}

fn main() {
    unsafe {
        signal(SIGINT, handle_signal as libc::sighandler_t);
    }

    for _ in 0..5 {
        println!("hello");
        sleep(Duration::from_secs(1));
    }
}
