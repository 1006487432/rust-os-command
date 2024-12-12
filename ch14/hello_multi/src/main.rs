use std::thread;
use std::time::Duration;
use std::io::Write;
const NUM: u32 = 5;

fn main() {
    let t1 = thread::spawn(|| {
        print_msg("hello");
    });

    let t2 = thread::spawn(|| {
        print_msg("world\n");
    });

    t1.join().unwrap();
    t2.join().unwrap();
}

fn print_msg(msg: &str) {
    for _ in 0..NUM {
        print!("{}", msg);
        std::io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
    }
}
