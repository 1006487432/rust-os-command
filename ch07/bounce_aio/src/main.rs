use ncurses::*;
use libc::{c_int, sighandler_t, SIGALRM, SIGIO, SIGEV_SIGNAL, pause, signal};
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::ptr;

const MESSAGE: &str = "hello";
const BLANK: &str = "     ";

struct PPBall {
    row: AtomicI32,
    col: AtomicI32,
    dir: AtomicI32,
    done: AtomicBool,
    input: Mutex<[u8; 1]>,
}

extern "C" fn on_input(_sig: c_int) {
    unsafe {
        if let Some(the_ball) = BALL.as_ref() {
            let mut input = the_ball.input.lock().unwrap();
            if input[0] == 'Q' as u8 || input[0] == '\x04' as u8 {
                the_ball.done.store(true, Ordering::SeqCst);
            } else if input[0] == ' ' as u8 {
                the_ball.dir.store(-the_ball.dir.load(Ordering::SeqCst), Ordering::SeqCst);
            }
            *input = [0u8; 1];
            setup_aio_buffer(the_ball.clone());
        }
    }
}

extern "C" fn on_alarm(_sig: c_int) {
    unsafe {
        if let Some(the_ball) = BALL.as_ref() {
            mvaddstr(the_ball.row.load(Ordering::SeqCst), the_ball.col.load(Ordering::SeqCst), BLANK);
            the_ball.col.fetch_add(the_ball.dir.load(Ordering::SeqCst), Ordering::SeqCst);
            mvaddstr(the_ball.row.load(Ordering::SeqCst), the_ball.col.load(Ordering::SeqCst), MESSAGE);
            refresh();

            if the_ball.dir.load(Ordering::SeqCst) == -1 && the_ball.col.load(Ordering::SeqCst) <= 0 {
                the_ball.dir.store(1, Ordering::SeqCst);
            } else if the_ball.dir.load(Ordering::SeqCst) == 1 && the_ball.col.load(Ordering::SeqCst) + MESSAGE.len() as i32 >= COLS() {
                the_ball.dir.store(-1, Ordering::SeqCst);
            }
        }
    }
}

static mut BALL: Option<Arc<PPBall>> = None;

fn main() {
    let the_ball = Arc::new(PPBall {
        row: AtomicI32::new(10),
        col: AtomicI32::new(0),
        dir: AtomicI32::new(1),
        done: AtomicBool::new(false),
        input: Mutex::new([0u8; 1]),
    });

    unsafe {
        BALL = Some(the_ball.clone());
    }

    initscr();
    cbreak();
    noecho();
    clear();

    mvaddstr(the_ball.row.load(Ordering::SeqCst), the_ball.col.load(Ordering::SeqCst), MESSAGE);
    refresh();

    setup_aio_buffer(the_ball.clone());

    let ball_clone = the_ball.clone();
    thread::spawn(move || {
        loop {
            ball_move(&ball_clone);
            thread::sleep(Duration::from_millis(1000 / 60));
        }
    });

    unsafe {
        signal(SIGIO, on_input as sighandler_t);
        signal(SIGALRM, on_alarm as sighandler_t);
    }

    while !the_ball.done.load(Ordering::SeqCst) {
        unsafe {
            pause();
        }
    }

    endwin();
}

fn setup_aio_buffer(the_ball: Arc<PPBall>) {
    let mut input = the_ball.input.lock().unwrap();
    *input = [0u8; 1];
    drop(input);

    let mut kbcbuf: libc::aiocb = unsafe { std::mem::zeroed() };
    kbcbuf.aio_fildes = 0;
    kbcbuf.aio_buf = the_ball.input.lock().unwrap().as_mut_ptr() as *mut _;
    kbcbuf.aio_nbytes = 1;
    kbcbuf.aio_offset = 0;
    kbcbuf.aio_sigevent.sigev_notify = SIGEV_SIGNAL;
    kbcbuf.aio_sigevent.sigev_signo = SIGIO;
    kbcbuf.aio_sigevent.sigev_value.sival_ptr = ptr::null_mut();

    unsafe {
        libc::aio_read(&mut kbcbuf);
    }
}

fn ball_move(the_ball: &Arc<PPBall>) {
    mvaddstr(the_ball.row.load(Ordering::SeqCst), the_ball.col.load(Ordering::SeqCst), BLANK);
    the_ball.col.fetch_add(the_ball.dir.load(Ordering::SeqCst), Ordering::SeqCst);
    mvaddstr(the_ball.row.load(Ordering::SeqCst), the_ball.col.load(Ordering::SeqCst), MESSAGE);
    refresh();

    if the_ball.dir.load(Ordering::SeqCst) == -1 && the_ball.col.load(Ordering::SeqCst) <= 0 {
        the_ball.dir.store(1, Ordering::SeqCst);
    } else if the_ball.dir.load(Ordering::SeqCst) == 1 && the_ball.col.load(Ordering::SeqCst) + MESSAGE.len() as i32 >= COLS() {
        the_ball.dir.store(-1, Ordering::SeqCst);
    }
}
