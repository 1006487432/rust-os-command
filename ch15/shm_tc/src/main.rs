// shm_tc.rs : the time client using shared memory, a bizarre application

use std::ffi::CStr;
use std::ptr;
use libc::{shmat, shmget, shmdt, IPC_CREAT, SHM_R, SHM_W, MAP_FAILED};

const TIME_MEM_KEY: i32 = 99;       // kind of like a port number
const SEG_SIZE: usize = 100;        // size of segment

fn oops(message: &str, code: i32) {
    eprintln!("{}: {}", message, std::io::Error::last_os_error());
    std::process::exit(code);
}

fn main() {
    // create a shared memory segment
    let seg_id = unsafe { shmget(TIME_MEM_KEY, SEG_SIZE, SHM_R | SHM_W | IPC_CREAT) };
    if seg_id == -1 {
        oops("shmget", 1);
    }

    // attach to it and get a pointer to where it attaches
    let mem_ptr = unsafe { shmat(seg_id, ptr::null(), 0) };
    if mem_ptr == MAP_FAILED {
        oops("shmat", 2);
    }

    // read the time from shared memory
    unsafe {
        let c_str = CStr::from_ptr(mem_ptr as *const _);
        if let Ok(str_slice) = c_str.to_str() {
            println!("The time, direct from memory: ..{}", str_slice);
        } else {
            oops("Reading shared memory", 3);
        }
    }

    // detach, but not needed here
    unsafe { shmdt(mem_ptr) };
}
