use std::io::{self, Read};
use std::os::unix::io::AsRawFd;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;
use libc::{signal, SIGINT, SIGQUIT};
use nix::unistd::read;

const INPUTLEN: usize = 100;

extern "C" fn inthandler(sig: libc::c_int) {
    println!(" Received signal {} .. waiting", sig);
    sleep(Duration::from_secs(2));
    println!("  Leaving inthandler");
}

extern "C" fn quithandler(sig: libc::c_int) {
    println!(" Received signal {} .. waiting", sig);
    sleep(Duration::from_secs(3));
    println!("  Leaving quithandler");
}

fn main() {
    unsafe {
        signal(SIGINT, inthandler as libc::sighandler_t);
        signal(SIGQUIT, quithandler as libc::sighandler_t);
    }

    let mut input = [0u8; INPUTLEN];

    loop {
        println!("\nType a message");

        match read(io::stdin().as_raw_fd(), &mut input) {
            Ok(nchars) if nchars > 0 => {
                let input_str = String::from_utf8_lossy(&input[..nchars]);
                println!("You typed: {}", input_str);
                if input_str.trim() == "quit" {
                    break;
                }
            }
            Ok(_) => eprintln!("No input read"),
            Err(e) => {
                eprintln!("read returned an error: {}", e);
                exit(1);
            }
        }
    }
}
