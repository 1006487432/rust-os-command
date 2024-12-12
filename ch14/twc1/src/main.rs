use std::fs::File;
use std::io::{self, BufRead};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: {} file1 file2", args[0]);
        std::process::exit(1);
    }

    let total_words = Arc::new(Mutex::new(0));
    let files = vec![args[1].clone(), args[2].clone()];

    let handles: Vec<_> = files.into_iter().map(|filename| {
        let total_words = Arc::clone(&total_words);
        thread::spawn(move || {
            count_words(filename, total_words);
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }

    let total_words = total_words.lock().unwrap();
    println!("{:5}: total words", *total_words);
}

fn count_words(filename: String, total_words: Arc<Mutex<i32>>) {
    let file = match File::open(&filename) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening file {}: {}", filename, err);
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

    let mut total_words = total_words.lock().unwrap();
    *total_words += count;
}
