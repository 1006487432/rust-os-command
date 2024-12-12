use termios::*;

fn main() {
    match Termios::from_fd(0) {
        Ok(termios) =>{
            if termios.c_lflag & ECHO == ECHO {
                println!("echo is on, since its bit is 1");
            }else {
                println!("echo is OFF, since its bit is 0");
            }
        },
        Err(e) =>{
            eprintln!("{}", e);
        }
    } ;
}
