use termios::*;

fn main() {
    let args:Vec<String>= std::env::args().collect();
    if args.len() == 1{
        return;
    }
    match Termios::from_fd(0) {
        Ok(mut termios) =>{
            if args[1].as_bytes()[0] == u8::try_from('y').unwrap() {
                termios.c_lflag |= ECHO;
            }else{
                termios.c_lflag &= !ECHO;
            }
            tcsetattr(0, TCSANOW, &mut termios).expect("更改失败");
        },
        Err(e) =>{
            eprintln!("{}", e);
        }
    } ;
}
