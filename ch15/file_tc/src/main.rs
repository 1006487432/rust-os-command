use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::os::unix::io::AsRawFd;
use nix::fcntl::{FcntlArg, FlockArg, flock};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: file_tc filename");
        std::process::exit(1);
    }

    let filename = &args[1];
    let mut file = File::open(filename).unwrap_or_else(|err| {
        eprintln!("{}: {}", filename, err);
        std::process::exit(3);
    });

    lock_operation(file.as_raw_fd(), FlockArg::LockShared).unwrap();

    let mut buf = [0; 10];
    while let Ok(nread) = file.read(&mut buf) {
        if nread == 0 {
            break;
        }
        io::stdout().write_all(&buf[..nread]).unwrap();
    }

    lock_operation(file.as_raw_fd(), FlockArg::Unlock).unwrap();

    // File is automatically closed when it goes out of scope
}

fn lock_operation(fd: i32, op: FlockArg) -> Result<(), nix::Error> {
    flock(fd, op)?;
    Ok(())
}
