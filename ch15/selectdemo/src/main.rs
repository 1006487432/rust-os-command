use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::os::unix::io::AsRawFd;
use nix::sys::select::{select, FdSet};
use nix::sys::time::{TimeVal, TimeValLike};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("usage: {} file1 file2 timeout", args[0]);
        std::process::exit(1);
    }

    let file1 = &args[1];
    let file2 = &args[2];
    let timeout_sec: i64 = args[3].parse().unwrap_or_else(|_| {
        eprintln!("Invalid timeout value");
        std::process::exit(1);
    });

    let f1 = File::open(file1).unwrap_or_else(|err| {
        eprintln!("{}: {}", file1, err);
        std::process::exit(2);
    });
    let fd1 = f1.as_raw_fd();

    let f2 = File::open(file2).unwrap_or_else(|err| {
        eprintln!("{}: {}", file2, err);
        std::process::exit(3);
    });
    let fd2 = f2.as_raw_fd();

    let maxfd = 1 + fd1.max(fd2);
    let mut timeout = TimeVal::seconds(timeout_sec);

    loop {
        let mut readfds = FdSet::new();
        readfds.insert(fd1);
        readfds.insert(fd2);

        let result = select(maxfd + 1, &mut readfds, None, None, &mut timeout);

        match result {
            Ok(0) => println!("no input after {} seconds", timeout_sec),
            Ok(_) => {
                if readfds.contains(fd1) {
                    show_data(file1, &f1);
                }
                if readfds.contains(fd2) {
                    show_data(file2, &f2);
                }
            },
            Err(e) => {
                eprintln!("select: {}", e);
                break;
            }
        }
    }
}

fn show_data(filename: &str, mut file: &File) {
    let mut buf = [0; 1024];
    let n = file.read(&mut buf).expect("Failed to read file");
    print!("{}: {}", filename, std::str::from_utf8(&buf[..n]).unwrap());
    io::stdout().flush().unwrap();
}
