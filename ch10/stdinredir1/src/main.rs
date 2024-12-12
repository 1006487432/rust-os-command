use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::process::exit;
use nix::unistd::close;

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut line = String::new();

    // 读取并打印三行标准输入
    for _ in 0..3 {
        handle.read_line(&mut line).expect("Failed to read line");
        print!("{}", line);
        line.clear();
    }

    // 重定向输入
    close(0).expect("Failed to close stdin");
    let fd = File::open("/etc/passwd").expect("Could not open data as fd 0");
    if fd.as_raw_fd() != 0 {
        eprintln!("Could not open data as fd 0");
        exit(1);
    }

    // 读取并打印文件中的三行
    let file = unsafe { File::from_raw_fd(0) };
    let reader = BufReader::new(file);
    for line in reader.lines().take(3) {
        println!("{}", line.expect("Failed to read line"));
    }
}
