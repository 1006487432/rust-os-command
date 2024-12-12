use ncurses::*;
use std::sync::{Arc};
use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;
use std::time::Duration;

const MESSAGE: char = 'o';
const BLANK: char = ' ';

struct PPBall {
    y_pos: AtomicI32,
    x_pos: AtomicI32,
    y_ttg: AtomicI32,
    y_ttm: AtomicI32,
    x_ttg: AtomicI32,
    x_ttm: AtomicI32,
    y_dir: AtomicI32,
    x_dir: AtomicI32,
    symbol: char,
}

fn main() {
    let the_ball = Arc::new(PPBall {
        y_pos: AtomicI32::new(10),
        x_pos: AtomicI32::new(10),
        y_ttg: AtomicI32::new(5),
        y_ttm: AtomicI32::new(5),
        x_ttg: AtomicI32::new(5),
        x_ttm: AtomicI32::new(5),
        y_dir: AtomicI32::new(1),
        x_dir: AtomicI32::new(1),
        symbol: MESSAGE,
    });

    let the_ball_clone = Arc::clone(&the_ball);

    initscr();
    cbreak();
    noecho();
    clear();

    mvaddch(the_ball.y_pos.load(Ordering::SeqCst), the_ball.x_pos.load(Ordering::SeqCst), the_ball.symbol as u32);
    refresh();

    let handle = thread::spawn(move || {
        loop {
            ball_move(&the_ball_clone);
            thread::sleep(Duration::from_millis(1000 / 60));
        }
    });

    loop {
        let c = getch();
        if c == 'Q' as i32 {
            break;
        }
        if c == 'f' as i32 {
            the_ball.x_ttm.fetch_sub(1, Ordering::SeqCst);
        } else if c == 's' as i32 {
            the_ball.x_ttm.fetch_add(1, Ordering::SeqCst);
        } else if c == 'F' as i32 {
            the_ball.y_ttm.fetch_sub(1, Ordering::SeqCst);
        } else if c == 'S' as i32 {
            the_ball.y_ttm.fetch_add(1, Ordering::SeqCst);
        }
    }

    wrap_up();
    handle.join().unwrap();
}

fn wrap_up() {
    endwin();
}

fn ball_move(the_ball: &Arc<PPBall>) {
    let y_cur = the_ball.y_pos.load(Ordering::SeqCst);
    let x_cur = the_ball.x_pos.load(Ordering::SeqCst);
    let mut moved = false;

    if the_ball.y_ttm.load(Ordering::SeqCst) > 0 && the_ball.y_ttg.fetch_sub(1, Ordering::SeqCst) == 1 {
        the_ball.y_pos.fetch_add(the_ball.y_dir.load(Ordering::SeqCst), Ordering::SeqCst);
        the_ball.y_ttg.store(the_ball.y_ttm.load(Ordering::SeqCst), Ordering::SeqCst);
        moved = true;
    }

    if the_ball.x_ttm.load(Ordering::SeqCst) > 0 && the_ball.x_ttg.fetch_sub(1, Ordering::SeqCst) == 1 {
        the_ball.x_pos.fetch_add(the_ball.x_dir.load(Ordering::SeqCst), Ordering::SeqCst);
        the_ball.x_ttg.store(the_ball.x_ttm.load(Ordering::SeqCst), Ordering::SeqCst);
        moved = true;
    }

    if moved {
        mvaddch(y_cur, x_cur, BLANK as u32);
        mvaddch(the_ball.y_pos.load(Ordering::SeqCst), the_ball.x_pos.load(Ordering::SeqCst), the_ball.symbol as u32);
        bounce_or_lose(&the_ball);
        mv(LINES() - 1, COLS() - 1);
        refresh();
    }
}

fn bounce_or_lose(the_ball: &Arc<PPBall>) -> i32 {
    let mut return_val = 0;

    if the_ball.y_pos.load(Ordering::SeqCst) <= 0 {
        the_ball.y_dir.store(1, Ordering::SeqCst);
        return_val = 1;
    } else if the_ball.y_pos.load(Ordering::SeqCst) >= LINES() - 1 {
        the_ball.y_dir.store(-1, Ordering::SeqCst);
        return_val = 1;
    }

    if the_ball.x_pos.load(Ordering::SeqCst) <= 0 {
        the_ball.x_dir.store(1, Ordering::SeqCst);
        return_val = 1;
    } else if the_ball.x_pos.load(Ordering::SeqCst) >= COLS() - 1 {
        the_ball.x_dir.store(-1, Ordering::SeqCst);
        return_val = 1;
    }

    return return_val;
}
