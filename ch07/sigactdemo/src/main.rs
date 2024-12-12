use std::io::{self, Write};
use std::time::Duration;
use std::thread::sleep;
use libc::{sigaction, sigaddset, sigemptyset, sighandler_t, sigset_t, SIGINT, SIGQUIT, SA_RESETHAND, SA_RESTART};

extern "C" fn inthandler(signum: libc::c_int) {
    println!("Called with signal {}", signum);
    sleep(Duration::from_secs(signum as u64));
    println!("done handling signal {}", signum);
}

fn main() {
    let mut newhandler: libc::sigaction = unsafe { std::mem::zeroed() };
    let mut blocked: sigset_t = unsafe { std::mem::zeroed() };

    unsafe {
        newhandler.sa_sigaction = inthandler as sighandler_t;
        newhandler.sa_flags = SA_RESETHAND | SA_RESTART;

        sigemptyset(&mut blocked);
        sigaddset(&mut blocked, SIGQUIT);

        newhandler.sa_mask = blocked;

        if sigaction(SIGINT, &newhandler, std::ptr::null_mut()) == -1 {
            eprintln!("sigaction error");
        } else {
            loop {
                let mut input = String::new();
                io::stdout().flush().unwrap();
                match io::stdin().read_line(&mut input) {
                    Ok(_) => println!("input: {}", input),
                    Err(e) => eprintln!("Error reading input: {}", e),
                }
            }
        }
    }
}
