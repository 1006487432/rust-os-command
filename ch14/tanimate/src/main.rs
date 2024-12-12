use ncurses::*;
use rand::Rng;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const MAXMSG: usize = 10;
const TUNIT: u64 = 20000;

struct PropSet {
    str: String,
    row: i32,
    delay: u64,
    dir: i32,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        println!("usage: tanimate string ..");
        return;
    }

    let num_msg = args.len() - 1;
    let props: Arc<Mutex<Vec<PropSet>>> = Arc::new(Mutex::new(Vec::new()));
    for i in 0..num_msg {
        props.lock().unwrap().push(PropSet {
            str: args[i + 1].clone(),
            row: i as i32,
            delay: 1 + rand::thread_rng().gen_range(0..15) as u64,
            dir: if rand::random::<bool>() { 1 } else { -1 },
        });
    }

    initscr();
    raw();
    noecho();
    clear();

    let props_clone = Arc::clone(&props);
    let thrds: Vec<_> = props_clone
        .lock()
        .unwrap()
        .iter()
        .enumerate()
        .map(|(i, _)| {
            let props_clone = Arc::clone(&props_clone);
            thread::spawn(move || animate(props_clone, i))
        })
        .collect();

    loop {
        let c = getch();
        if c == 'Q' as i32 {
            break;
        }
        if c == ' ' as i32 {
            let mut props = props.lock().unwrap();
            for prop in props.iter_mut() {
                prop.dir = -prop.dir;
            }
        }
        if c >= '0' as i32 && c <= '9' as i32 {
            let i = (c - '0' as i32) as usize;
            if i < num_msg {
                let mut props = props.lock().unwrap();
                props[i].dir = -props[i].dir;
            }
        }
    }

    endwin();
    for thrd in thrds {
        thrd.join().unwrap();
    }
}

fn animate(props: Arc<Mutex<Vec<PropSet>>>, index: usize) {
    let len: i32;
    let mut col: i32;
    {
        let props = props.lock().unwrap();
        len = props[index].str.len() as i32 + 2;
        col = rand::thread_rng().gen_range(0..(COLS() - len - 3)) as i32;
    }

    loop {
        let delay;
        {
            let props = props.lock().unwrap();
            delay = props[index].delay;
        }
        thread::sleep(Duration::from_micros(delay * TUNIT));

        {
            let props = props.lock().unwrap();
            mv(props[index].row, col);
            addch(' ' as u32);
            addstr(&props[index].str);
            addch(' ' as u32);
            mv(LINES() - 1, COLS() - 1);
            refresh();
        }

        col += props.lock().unwrap()[index].dir;

        let mut props = props.lock().unwrap();
        if col <= 0 && props[index].dir == -1 {
            props[index].dir = 1;
        } else if col + len >= COLS() && props[index].dir == 1 {
            props[index].dir = -1;
        }
    }
}
