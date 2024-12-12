use std::process::Command;

fn main() {
    let arglist = ["-l"];
    println!("* * * About to exec ls -l");
    let status = Command::new("ls")
        .args(&arglist)
        .status()
        .expect("failed to execute process");
    if status.success() {
        println!("* * * ls is done. bye");
    } else {
        println!("* * * ls did not complete successfully");
    }
}
