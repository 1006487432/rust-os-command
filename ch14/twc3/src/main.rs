use std::fs::File;
use std::io::{self, BufRead};
use std::sync::{Arc, Mutex};
use std::thread;

struct ArgSet {
    fname: String,
    count: Arc<Mutex<i32>>,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: {} file1 file2", args[0]);
        std::process::exit(1);
    }

    let count1 = Arc::new(Mutex::new(0));
    let count2 = Arc::new(Mutex::new(0));

    let args1 = ArgSet {
        fname: args[1].clone(),
        count: Arc::clone(&count1),
    };
    let args2 = ArgSet {
        fname: args[2].clone(),
        count: Arc::clone(&count2),
    };

    let handle1 = thread::spawn(move || {
        count_words(args1);
    });
    let handle2 = thread::spawn(move || {
        count_words(args2);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let count1 = count1.lock().unwrap();
    let count2 = count2.lock().unwrap();
    println!("{:5}: {}", *count1, args[1]);
    println!("{:5}: {}", *count2, args[2]);
    println!("{:5}: total words", *count1 + *count2);
}

fn count_words(arg: ArgSet) {
    let file = match File::open(&arg.fname) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening file {}: {}", arg.fname, err);
            return;
        }
    };

    let mut prev_c = '\0';
    let mut count = 0;
    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        for c in line.chars() {
            if !c.is_alphanumeric() && prev_c.is_alphanumeric() {
                count += 1;
            }
            prev_c = c;
        }
    }

    let mut total_words = arg.count.lock().unwrap();
    *total_words += count;
}
