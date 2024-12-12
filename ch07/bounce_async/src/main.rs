use ncurses::*;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::sync::Arc;
use libc::{c_int, sighandler_t, SIGALRM, SIGIO, fcntl, F_SETFL, F_SETOWN, F_GETFL, O_ASYNC, getpid, pause, signal, EOF, ITIMER_REAL};
use std::ptr;
use libc::itimerval;

extern "C" { fn setitimer(which: libc::c_int, new_value: *const itimerval, old_value: *mut itimerval) -> libc::c_int; }

const MESSAGE: &str = "hello";
const BLANK: &str = "     ";

struct State {
    row: AtomicI32,
    col: AtomicI32,
    dir: AtomicI32,
    delay: AtomicI32,
    done: AtomicBool,
}

extern "C" fn on_input(_sig: c_int) {
    unsafe {
        if let Some(state) = STATE.as_ref() {
            let c = getch();
            if c == 'Q' as i32 || c == EOF {
                state.done.store(true, Ordering::SeqCst);
            } else if c == ' ' as i32 {
                state.dir.store(-state.dir.load(Ordering::SeqCst), Ordering::SeqCst);
            }
        }
    }
}

extern "C" fn on_alarm(_sig: c_int) {
    unsafe {
        if let Some(state) = STATE.as_ref() {
            mvaddstr(state.row.load(Ordering::SeqCst), state.col.load(Ordering::SeqCst), BLANK);
            state.col.fetch_add(state.dir.load(Ordering::SeqCst), Ordering::SeqCst);
            mvaddstr(state.row.load(Ordering::SeqCst), state.col.load(Ordering::SeqCst), MESSAGE);
            refresh();

            if state.dir.load(Ordering::SeqCst) == -1 && state.col.load(Ordering::SeqCst) <= 0 {
                state.dir.store(1, Ordering::SeqCst);
            } else if state.dir.load(Ordering::SeqCst) == 1 && state.col.load(Ordering::SeqCst) + MESSAGE.len() as i32 >= COLS() {
                state.dir.store(-1, Ordering::SeqCst);
            }
        }
    }
    unsafe { signal(SIGALRM, on_alarm as sighandler_t) };
}

static mut STATE: Option<Arc<State>> = None;

fn main() {
    let state = Arc::new(State {
        row: AtomicI32::new(10),
        col: AtomicI32::new(0),
        dir: AtomicI32::new(1),
        delay: AtomicI32::new(200),
        done: AtomicBool::new(false),
    });

    unsafe {
        STATE = Some(state.clone());
    }

    initscr();
    cbreak();
    noecho();
    clear();

    mvaddstr(state.row.load(Ordering::SeqCst), state.col.load(Ordering::SeqCst), MESSAGE);
    refresh();

    unsafe {
        signal(SIGIO, on_input as sighandler_t);
        enable_kbd_signals();
        signal(SIGALRM, on_alarm as sighandler_t);
    }

    set_ticker(state.delay.load(Ordering::SeqCst));

    while !state.done.load(Ordering::SeqCst) {
        unsafe {
            pause();
        }
    }

    endwin();
}

fn enable_kbd_signals() {
    unsafe {
        let pid = getpid();
        fcntl(0, F_SETOWN, pid);
        let fd_flags = fcntl(0, F_GETFL);
        fcntl(0, F_SETFL, fd_flags | O_ASYNC);
    }
}

extern "C" fn set_ticker(n_msecs: i32) {
    let mut new_timeset: libc::itimerval = unsafe { std::mem::zeroed() };
    let n_sec = n_msecs / 1000;
    let n_usecs = (n_msecs % 1000) * 1000;

    new_timeset.it_interval.tv_sec = n_sec as libc::time_t;
    new_timeset.it_interval.tv_usec = n_usecs as libc::suseconds_t;
    new_timeset.it_value.tv_sec = n_sec as libc::time_t;
    new_timeset.it_value.tv_usec = n_usecs as libc::suseconds_t;

    unsafe {
        setitimer(ITIMER_REAL, &new_timeset, ptr::null_mut());
    }
}
