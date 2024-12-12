use std::process::Command;
use std::io::{self, Write};

const MAXARGS: usize = 20;
const ARGLEN: usize = 100;

fn main() {
    let mut arglist: Vec<String> = Vec::with_capacity(MAXARGS + 1);
    let mut numargs = 0;

    while numargs < MAXARGS {
        print!("Arg[{}]? ", numargs);
        io::stdout().flush().unwrap();

        let mut argbuf = String::with_capacity(ARGLEN);
        io::stdin().read_line(&mut argbuf).expect("Failed to read line");

        if argbuf.trim() != "" {
            arglist.push(makestring(argbuf));
            numargs += 1;
        } else {
            if numargs > 0 {
                arglist.push(String::new()); // 结束列表
                execute(&arglist);
                arglist.clear();
                numargs = 0;
            }
        }
    }
}

fn execute(arglist: &[String]) {
    let args: Vec<&str> = arglist.iter().map(|s| s.as_str()).collect();
    let status = Command::new(&args[0])
        .args(&args[1..])
        .status()
        .expect("Failed to execute command");

    if !status.success() {
        eprintln!("execvp failed");
        std::process::exit(1);
    }
}

fn makestring(buf: String) -> String {
    buf.trim().to_string()
}
