use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const NUM: u32 = 5;

fn main() {
    let counter = Arc::new(Mutex::new(0));

    let counter_clone = Arc::clone(&counter);
    let t1 = thread::spawn(move || {
        for _ in 0..NUM {
            {
                let count = counter_clone.lock().unwrap();
                println!("count = {}", count);
            }
            thread::sleep(Duration::from_secs(1));
        }
    });

    for _ in 0..NUM {
        {
            let mut count = counter.lock().unwrap();
            *count += 1;
        }
        thread::sleep(Duration::from_secs(1));
    }

    t1.join().unwrap();
}
