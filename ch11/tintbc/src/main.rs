use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::process::{Command};
use nix::unistd::{pipe, fork, dup2, close, ForkResult};
use std::os::unix::io::{FromRawFd, RawFd};

fn oops(message: &str, code: i32) -> ! {
    eprintln!("{}", message);
    std::process::exit(code);
}

fn be_dc(in_pipe: [RawFd; 2], out_pipe: [RawFd; 2]) {
    if dup2(in_pipe[0], 0).is_err() {
        oops("dc: cannot redirect stdin", 3);
    }
    close(in_pipe[0]).ok();
    close(in_pipe[1]).ok();

    if dup2(out_pipe[1], 1).is_err() {
        oops("dc: cannot redirect stdout", 4);
    }
    close(out_pipe[1]).ok();
    close(out_pipe[0]).ok();

    Command::new("dc")
        .arg("-")
        .spawn()
        .unwrap_or_else(|_| oops("Cannot run dc", 5));

    oops("This should not be printed if dc runs", 6);
}

fn be_bc(todc: [RawFd; 2], fromdc: [RawFd; 2]) {
    close(todc[0]).ok();
    close(fromdc[1]).ok();

    let stdin = io::stdin();

    // Convert file descriptors to BufWriter and BufReader
    let mut fpout = unsafe { BufWriter::new(File::from_raw_fd(todc[1])) };
    let mut fpin = unsafe { BufReader::new(File::from_raw_fd(fromdc[0])) };

    let mut input = String::new();
    while {
        print!("tinybc: ");
        io::stdout().flush().unwrap();
        input.clear();
        stdin.read_line(&mut input).unwrap() > 0
    } {
        let mut parts = input.split_whitespace();
        if let (Some(num1_str), Some(op_str), Some(num2_str)) = (parts.next(), parts.next(), parts.next()) {
            let num1: i32 = num1_str.parse().unwrap_or_else(|_| {
                println!("syntax error");
                0
            });
            let op: char = op_str.chars().next().unwrap_or(' ');
            let num2: i32 = num2_str.parse().unwrap_or_else(|_| {
                println!("syntax error");
                0
            });

            if op != ' ' {
                writeln!(fpout, "{}\n{}\n{}\np", num1, num2, op).unwrap();
                fpout.flush().unwrap();
                let mut output = String::new();
                fpin.read_line(&mut output).unwrap();
                println!("{} {} {} = {}", num1, op, num2, output.trim());
            } else {
                println!("syntax error");
            }
        } else {
            println!("syntax error");
        }
    }

    close(todc[1]).ok();
    close(fromdc[0]).ok();
}

fn main() {
    let todc = pipe().unwrap_or_else(|_| oops("pipe failed", 1));
    let fromdc = pipe().unwrap_or_else(|_| oops("pipe failed", 1));

    match unsafe { fork() } {
        Ok(ForkResult::Child) => be_dc(todc.into(), fromdc.into()),
        Ok(ForkResult::Parent { .. }) => {
            be_bc(todc.into(), fromdc.into());
            nix::sys::wait::wait().unwrap();
        }
        Err(_) => oops("cannot fork", 2),
    }
}
