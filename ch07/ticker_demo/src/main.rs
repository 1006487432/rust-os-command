use std::io::{self, Write};
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;
use libc::{signal, itimerval, SIGALRM};
use libc::ITIMER_REAL;
extern "C" { fn setitimer(which: libc::c_int, new_value: *const itimerval, old_value: *mut itimerval) -> libc::c_int; }
extern "C" fn countdown(signum: libc::c_int) {
    static mut NUM: i32 = 10;
    unsafe {
        println!("{} ..", NUM);
        io::stdout().flush().unwrap();
        NUM -= 1;
        if NUM < 0 {
            println!("DONE!");
            exit(0);
        }
    }
}

fn set_ticker(n_msecs: i32) -> i32 {
    let mut new_timeset = itimerval {
        it_interval: libc::timeval {
            tv_sec: (n_msecs / 1000) as libc::time_t,
            tv_usec: ((n_msecs % 1000) * 1000) as libc::suseconds_t,
        },
        it_value: libc::timeval {
            tv_sec: (n_msecs / 1000) as libc::time_t,
            tv_usec: ((n_msecs % 1000) * 1000) as libc::suseconds_t,
        },
    };

    unsafe {
        if setitimer(ITIMER_REAL, &new_timeset, std::ptr::null_mut()) == -1 {
            -1
        } else {
            0
        }
    }
}

fn main() {
    unsafe {
        signal(SIGALRM, countdown as libc::sighandler_t);
    }

    if set_ticker(500) == -1 {
        eprintln!("set_ticker error");
    } else {
        loop {
            sleep(Duration::from_secs(1));
        }
    }
}
