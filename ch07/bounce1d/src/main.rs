use ncurses::*;
use std::time::Duration;
use std::thread;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::{Arc, Mutex};

const MESSAGE: &str = "hello";
const BLANK: &str = "     ";

fn main() {
    let row: i32 = 10;
    let col = Arc::new(AtomicI32::new(0));
    let dir = Arc::new(AtomicI32::new(1));
    let delay = Arc::new(Mutex::new(Duration::from_millis(200)));

    initscr();
    cbreak();
    noecho();
    clear();

    // 显示初始消息
    mvprintw(row, col.load(Ordering::SeqCst), MESSAGE);
    refresh();

    let col_clone = Arc::clone(&col);
    let dir_clone = Arc::clone(&dir);
    let delay_clone = Arc::clone(&delay);

    let handle = thread::spawn(move || {
        loop {
            move_msg(row, &col_clone, &dir_clone);
            thread::sleep(*delay_clone.lock().unwrap());
        }
    });

    loop {
        let mut ndelay = Duration::new(0, 0);
        let c = getch();
        if c == 'Q' as i32 {
            break;
        }
        if c == ' ' as i32 {
            dir.store(-dir.load(Ordering::SeqCst), Ordering::SeqCst);
        }
        if c == 'f' as i32 {
            let mut delay = delay.lock().unwrap();
            if *delay > Duration::from_millis(2) {
                ndelay = *delay / 2;
            }
        }
        if c == 's' as i32 {
            let mut delay = delay.lock().unwrap();
            ndelay = *delay * 2;
        }
        if ndelay > Duration::new(0, 0) {
            *delay.lock().unwrap() = ndelay;
        }

        // 刷新屏幕以确保任何新的更改被显示
        refresh();
    }

    endwin();
    handle.join().unwrap();
}

fn move_msg(row: i32, col: &Arc<AtomicI32>, dir: &Arc<AtomicI32>) {
    mvprintw(row, col.load(Ordering::SeqCst), BLANK);
    col.store(col.load(Ordering::SeqCst) + dir.load(Ordering::SeqCst), Ordering::SeqCst);
    mvprintw(row, col.load(Ordering::SeqCst), MESSAGE);
    refresh();

    if dir.load(Ordering::SeqCst) == -1 && col.load(Ordering::SeqCst) <= 0 {
        dir.store(1, Ordering::SeqCst);
    } else if dir.load(Ordering::SeqCst) == 1 && (col.load(Ordering::SeqCst) + MESSAGE.len() as i32) >= COLS() {
        dir.store(-1, Ordering::SeqCst);
    }
}
