use std::env;
use std::fs::OpenOptions;
use std::io::{self, Seek, SeekFrom, Write};
use std::os::unix::io::AsRawFd;
use nix::fcntl::{flock, FlockArg};
use std::thread;
use std::time::Duration;
use chrono::Local;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: file_ts filename");
        std::process::exit(1);
    }

    let filename = &args[1];
    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(filename)
        .unwrap_or_else(|err| {
            eprintln!("{}: {}", filename, err);
            std::process::exit(2);
        });

    loop {
        let now = Local::now();
        let message = now.to_rfc2822();

        lock_operation(file.as_raw_fd(), FlockArg::LockExclusive).unwrap();

        file.seek(SeekFrom::Start(0)).unwrap();
        file.write_all(message.as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();
        file.flush().unwrap();

        lock_operation(file.as_raw_fd(), FlockArg::Unlock).unwrap();

        thread::sleep(Duration::from_secs(1));
    }
}

fn lock_operation(fd: i32, op: FlockArg) -> Result<(), nix::Error> {
    flock(fd, op)?;
    Ok(())
}
