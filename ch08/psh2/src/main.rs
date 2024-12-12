use std::process::{Command, exit};
use std::io::{self, Write};
use nix::unistd::{fork, ForkResult};
use nix::sys::wait::waitpid;
use std::ptr;
use std::os::unix::process::CommandExt;

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
    match unsafe { fork() } {
        Ok(ForkResult::Parent { child }) => {
            let mut exitstatus = 0;
            waitpid(child, None).expect("Failed to wait on child");
            println!("child exited with status {}", exitstatus);
        }
        Ok(ForkResult::Child) => {
            let args: Vec<&str> = arglist.iter().map(|s| s.as_str()).collect();
            Command::new(&args[0])
                .args(&args[1..])
                .exec();
            eprintln!("execvp failed");
            exit(1);
        }
        Err(err) => {
            eprintln!("Fork failed: {}", err);
            exit(1);
        }
    }
}

fn makestring(buf: String) -> String {
    buf.trim().to_string()
}
