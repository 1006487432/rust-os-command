use std::io::{self, Write};
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;
use libc::{signal, alarm, pause, SIGALRM};

extern "C" fn wakeup(signum: libc::c_int) {
    #[cfg(not(feature = "shhhh"))]
    println!("Alarm received from kernel");
}

fn main() {
    unsafe {
        signal(SIGALRM, wakeup as libc::sighandler_t);
    }

    println!("about to sleep for 4 seconds");
    unsafe {
        alarm(4);
        pause();
    }
    println!("Morning so soon?");
}
