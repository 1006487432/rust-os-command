use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let ac = args.len();

    println!("Number of args: {}, Args are:", ac);
    for (i, arg) in args.iter().enumerate() {
        println!("args[{}] {}", i, arg);
    }

    eprintln!("This message is sent to stderr.");
}
