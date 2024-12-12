use ncurses::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const MESSAGE: &str = " hello ";

struct SharedData {
    row: i32,
    col: i32,
    dir: i32,
    delay: u64,
}

fn main() {
    let shared_data = Arc::new(Mutex::new(SharedData {
        row: 10,
        col: 0,
        dir: 1,
        delay: 200,
    }));

    initscr();
    raw();
    noecho();
    clear();

    let shared_data_clone = Arc::clone(&shared_data);
    let msg_thread = thread::spawn(move || moving_msg(shared_data_clone));

    loop {
        let mut ndelay = 0;
        let c = getch();
        if c == 'Q' as i32 {
            break;
        }
        let mut data = shared_data.lock().unwrap();
        if c == ' ' as i32 {
            data.dir = -data.dir;
        } else if c == 'f' as i32 && data.delay > 2 {
            ndelay = data.delay / 2;
        } else if c == 's' as i32 {
            ndelay = data.delay * 2;
        }
        if ndelay > 0 {
            data.delay = ndelay;
        }
    }

    msg_thread.join().unwrap();
    endwin();
}

fn moving_msg(shared_data: Arc<Mutex<SharedData>>) {
    loop {
        let delay;
        {
            let data = shared_data.lock().unwrap();
            delay = data.delay;
        }
        thread::sleep(Duration::from_millis(delay));

        let (row, col, dir, len);
        {
            let mut data = shared_data.lock().unwrap();
            row = data.row;
            col = data.col;
            dir = data.dir;
            len = MESSAGE.len() as i32;

            data.col += dir;
            if data.col <= 0 && dir == -1 {
                data.dir = 1;
            } else if data.col + len >= COLS() && dir == 1 {
                data.dir = -1;
            }
        }

        mv(row, col);
        addstr(MESSAGE);
        refresh();
    }
}
