use std::process::{Command, Stdio};
use std::io::Write;

fn main() {
    let mut child = Command::new("mail")
        .arg("admin")
        .arg("backup")
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to spawn mail command");

    if let Some(stdin) = child.stdin.as_mut() {
        stdin.write_all(b"Error with backup!!\n").expect("Failed to write to stdin");
    }

    child.wait().expect("Mail command wasn't running");
}
