use std::env;
use std::ffi::CString;
use std::os::raw::{c_char, c_void};
use std::ptr;
use libc::{execvp};

fn main() {
    // 设置环境变量
    env::set_var("TERM", "vt100");
    env::set_var("HOME", "/on/the/range");

    // 准备调用execvp
    let cmd = CString::new("env").expect("CString::new failed");
    let args: [&CString; 2] = [&cmd, &CString::new("").expect("CString::new failed")];
    let mut c_args: Vec<*const c_char> = args.iter().map(|arg| arg.as_ptr()).collect();
    c_args.push(ptr::null());

    // 执行env程序
    unsafe {
        execvp(cmd.as_ptr(), c_args.as_ptr());
    }
}
