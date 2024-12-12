use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};

fn main() {
    let mut child = Command::new("sh")
        .arg("-c")
        .arg("who | sort")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start command");

    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        for (i, line) in reader.lines().enumerate() {
            println!("{:3} {}", i, line.expect("Failed to read line"));
        }
    }

    let _ = child.wait().expect("Command wasn't running");
}
