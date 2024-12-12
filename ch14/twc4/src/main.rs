use std::fs::File;
use std::io::{self, BufRead};
use std::sync::{Arc, Mutex, Condvar};
use std::thread;

struct ArgSet {
    fname: String,
    count: i32,
}

struct SharedData {
    mailbox: Option<ArgSet>,
    reports_in: i32,
    total_words: i32,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: {} file1 file2", args[0]);
        std::process::exit(1);
    }

    let shared_data = Arc::new((Mutex::new(SharedData {
        mailbox: None,
        reports_in: 0,
        total_words: 0,
    }), Condvar::new()));

    let args1 = ArgSet {
        fname: args[1].clone(),
        count: 0,
    };
    let args2 = ArgSet {
        fname: args[2].clone(),
        count: 0,
    };

    let shared_data_clone1 = Arc::clone(&shared_data);
    let shared_data_clone2 = Arc::clone(&shared_data);

    let t1 = thread::spawn(move || {
        count_words(args1, shared_data_clone1);
    });
    let t2 = thread::spawn(move || {
        count_words(args2, shared_data_clone2);
    });

    let (lock, cvar) = &*shared_data;
    let mut data = lock.lock().unwrap();

    while data.reports_in < 2 {
        println!("MAIN: waiting for flag to go up");
        data = cvar.wait(data).unwrap();
        println!("MAIN: Wow! flag was raised, I have the lock");
        if let Some(mailbox) = &data.mailbox {
            println!("{:7}: {}", mailbox.count, mailbox.fname);
            data.total_words += mailbox.count;
        }
        data.mailbox = None;
        cvar.notify_all();
        data.reports_in += 1;
    }

    println!("{:7}: total words", data.total_words);

    t1.join().unwrap();
    t2.join().unwrap();
}

fn count_words(arg: ArgSet, shared_data: Arc<(Mutex<SharedData>, Condvar)>) {
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

    let (lock, cvar) = &*shared_data;
    let mut data = lock.lock().unwrap();
    data.mailbox = Some(ArgSet {
        fname: arg.fname.clone(),
        count,
    });

    println!("COUNT: raising flag");
    cvar.notify_all();

    while data.mailbox.is_some() {
        println!("COUNT: oops..mailbox not empty. wait for signal");
        data = cvar.wait(data).unwrap();
    }
    println!("COUNT: unlocking box");
}
