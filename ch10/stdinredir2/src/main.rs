use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::process::exit;
use nix::unistd::{close, dup, dup2};

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
    let fd = File::open("/etc/passwd").expect("Could not open data as fd 0");
    let newfd;

    // 在这里选择使用哪种方式进行重定向
    let use_dup2 = true; // 设置为 true 使用 dup2, 否则使用 close + dup

    if use_dup2 {
        newfd = dup2(fd.as_raw_fd(), 0).expect("Could not duplicate fd to 0 using dup2");
    } else {
        close(0).expect("Failed to close stdin");
        newfd = dup(fd.as_raw_fd()).expect("Could not duplicate fd to 0 using dup");
    }

    if newfd != 0 {
        eprintln!("Could not duplicate fd to 0");
        exit(1);
    }
    close(fd.as_raw_fd()).ok();

    // 读取并打印重定向后的三行
    let file = unsafe { File::from_raw_fd(0) };
    let reader = BufReader::new(file);
    for line in reader.lines().take(3) {
        println!("{}", line.expect("Failed to read line"));
    }
}
