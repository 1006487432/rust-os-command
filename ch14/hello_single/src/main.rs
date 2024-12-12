use std::io::Write;
use std::thread;
use std::time::Duration;

const NUM: u32 = 5;

fn main() {
    print_msg("hello");
    print_msg("world\n");
}

fn print_msg(msg: &str) {
    for _ in 0..NUM {
        print!("{}", msg);
        std::io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
    }
}
