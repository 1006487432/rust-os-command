use std::io::Write;
use std::process::exit;
use nix::unistd::pipe;

fn main() {
    let mut p = [0; 2];

    if pipe().is_err() {
        eprintln!("pipe failed");
        exit(1);
    }

    if nix::unistd::write(p[1], b"hello").is_err() {
        eprintln!("write into pipe[1] failed");
    } else {
        println!("write into pipe[1] worked");
    }
}
