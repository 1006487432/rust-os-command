use libc;
use libc::c_int;
use termios::*;
struct FlagInfo {
    fl_value: libc::c_int,
    fl_name: &'static str,
}
const INPUT_FLAGS: [FlagInfo; 10] = [
    FlagInfo { fl_value: libc::IGNBRK as libc::c_int, fl_name: "Ignore break condition" },
    FlagInfo { fl_value: libc::BRKINT as libc::c_int, fl_name: "Signal interrupt on break" },
    FlagInfo { fl_value: libc::IGNPAR as libc::c_int, fl_name: "Ignore chars with parity errors" },
    FlagInfo { fl_value: libc::PARMRK as libc::c_int, fl_name: "Mark parity errors" },
    FlagInfo { fl_value: libc::INPCK as libc::c_int, fl_name: "Enable input parity check" },
    FlagInfo { fl_value: libc::ISTRIP as libc::c_int, fl_name: "Strip character" },
    FlagInfo { fl_value: libc::INLCR as libc::c_int, fl_name: "Map NL to CR on input" },
    FlagInfo { fl_value: libc::IGNCR as libc::c_int, fl_name: "Ignore CR" },
    FlagInfo { fl_value: libc::ICRNL as libc::c_int, fl_name: "Map CR to NL on input" },
    FlagInfo { fl_value: libc::IXON as libc::c_int, fl_name: "Enable start/stop output control" },
];

const LOCAL_FLAGS: [FlagInfo; 5] = [
    FlagInfo { fl_value: libc::ISIG as libc::c_int, fl_name: "Enable signals" },
    FlagInfo { fl_value: libc::ICANON as libc::c_int, fl_name: "Canonical input (erase and kill)" },
    FlagInfo { fl_value: libc::ECHO as libc::c_int, fl_name: "Enable echo" },
    FlagInfo { fl_value: libc::ECHOE as libc::c_int, fl_name: "Echo ERASE as BS-SPACE-BS" },
    FlagInfo { fl_value: libc::ECHOK as libc::c_int, fl_name: "Echo KILL by starting new line" },
];

fn main() {
    match Termios::from_fd(0) {
        Ok(mut termios) =>{
            show_baud(cfgetospeed(&termios));
            println!("The erase character is ascii {}, Ctrl - {}", termios.c_cc[VERASE], (termios.c_cc[VERASE] -1 + ('A' as u8)) as char);
            println!("The line kill character is ascii {}, Ctrl - {}", termios.c_cc[VKILL], (termios.c_cc[VKILL] -1 + ('A' as u8)) as char);
            show_some_flags(&termios);
        },
        Err(e) =>{
            eprintln!("{}", e);
        }
    } ;
}

fn show_some_flags(info: &Termios) {
    show_flagset(info.c_iflag, &INPUT_FLAGS);
    show_flagset(info.c_lflag, &LOCAL_FLAGS);
}

fn show_flagset(thevalue: tcflag_t, thebitnames: &[FlagInfo]) {
    for i in thebitnames{
        print!("{} is ", i.fl_name);
        if (thevalue as c_int) & i.fl_value > 0{
            println!("ON");
        }else{
            println!("OFF");
        }
    }
}

fn show_baud(speed: speed_t){
    print!("the baud rate is ");
    match speed {
        B300 => println!("300"),
        B600 => println!("600"),
        B1200 => println!("1200"),
        B1800 => println!("1800"),
        B2400 => println!("2400"),
        B4800=> println!("4800"),
        B9600 => println!("9600"),
        _ => println!("Fast"),
    }
}