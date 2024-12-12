use std::io::{self, Write, Read};
use std::process::{exit, ExitCode};
use libc::{fcntl, sleep, F_GETFL, F_SETFL, O_NDELAY};
use termios::*;
static ASK: &str = "Do you wnat another transaction";
const TRIES: i32 = 3;
const SLEEPTIME: u32 = 2;
static mut STORED: i32 = 0;
static mut ORIGINAL_FLAGS: Option<Termios> = None;
fn main() -> ExitCode  {
    tty_mode(0);
    unsafe {
        libc::signal(libc::SIGINT, ctrl_c_handler as libc::sighandler_t); // 2
        libc::signal(libc::SIGQUIT, libc::SIG_IGN);
    }
    set_cr_noecho_mode();
    set_nodelay_mode();
    let response = get_response(ASK, &TRIES);
    tty_mode(1);
    if response == 2{
        ExitCode::FAILURE
    }else{
        ExitCode::SUCCESS
    }
}

fn get_response(question: &str, maxtries: &i32) -> i32 {
    println!("{}(y/n)", question);
    let mut tmp = maxtries.clone();
    io::stdout().flush().expect("panic message");
    loop {
        unsafe { sleep(SLEEPTIME); }
        let input = get_ok_char();
        if input == 'y' {
            return 0;
        }
        if input == 'n' {
            return 1;
        }
        if tmp == 0 {
            return 2;
        } else {
            tmp -= 1;
        }
        print!("\x07"); // 或者使用 "\a"
        io::stdout().flush();
    }
}
fn get_ok_char() -> char {
    let mut buffer = [0; 1];
    loop {
        match io::stdin().read(&mut buffer) {
            Ok(0) => {
                println!("Reached EOF.");
                break;
            },
            Ok(_) => {
                let c = buffer[0] as char;
                match c {
                    'y' => return 'y',
                    'Y' => return 'y',
                    'N' => return 'n',
                    'n' => return 'n',
                    _ => return '0'
                }
            },
            Err(e) => {
                //eprintln!("Error reading input: {}", e);
                break;
            }
        }
    }
    '0'
}
extern "C" fn ctrl_c_handler(sig: libc::c_int){
    tty_mode(1);
    //println!("Received SIGINT: {}", sig);
    exit(1);
}
fn tty_mode(how: i32){
    unsafe {
        if how == 0 {
            match Termios::from_fd(0) {
                Ok(mut termios) =>{
                    ORIGINAL_FLAGS = Some(termios);
                    STORED = 1;
                },
                Err(e) =>{
                    eprintln!("{}", e);
                }
            } ;
        } else if how == 1 && STORED == 1 {
            if let Some(ref original_termios) = ORIGINAL_FLAGS {
                tcsetattr(0, TCSANOW, original_termios).expect("RESTORED ERROR");
            }
        }
    }
}
fn set_cr_noecho_mode(){
    match Termios::from_fd(0) {
        Ok(mut termios) =>{
            termios.c_lflag &= !ICANON;
            termios.c_lflag &= !ECHO;
            termios.c_cc[VMIN] = 1;
            tcsetattr(0, TCSANOW, &termios).expect("修改失败");
        },
        Err(e) =>{
            eprintln!("{}", e);
        }
    }
}
fn set_nodelay_mode(){
    let mut termflags = unsafe { libc::fcntl(0, F_GETFL) };
    termflags |= O_NDELAY;
    unsafe{fcntl(0, F_SETFL, termflags)};
}
