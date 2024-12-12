use libc::{itimerval, ITIMER_REAL};
use std::mem::zeroed;
extern "C" { fn setitimer(which: libc::c_int, new_value: *const itimerval, old_value: *mut itimerval) -> libc::c_int; }

/// 设置定时器，使其在指定的毫秒数后触发 SIGALRM 信号
/// 返回 -1 表示出错，0 表示成功
fn set_ticker(n_msecs: i32) -> i32 {
    let mut new_timeset: itimerval = unsafe { zeroed() };

    let n_sec = n_msecs / 1000;
    let n_usecs = (n_msecs % 1000) * 1000;

    new_timeset.it_interval.tv_sec = n_sec as libc::time_t;
    new_timeset.it_interval.tv_usec = n_usecs as libc::suseconds_t;
    new_timeset.it_value.tv_sec = n_sec as libc::time_t;
    new_timeset.it_value.tv_usec = n_usecs as libc::suseconds_t;

    unsafe {
        if setitimer(ITIMER_REAL, &new_timeset, std::ptr::null_mut()) == -1 {
            -1
        } else {
            0
        }
    }
}

fn main() {
    // 调用 set_ticker 示例
    let result = set_ticker(500); // 设置定时器为 500 毫秒
    if result == -1 {
        eprintln!("set_ticker 出错");
    } else {
        println!("set_ticker 成功设置定时器");
    }
}
