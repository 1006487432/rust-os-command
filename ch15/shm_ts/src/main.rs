// shm_ts.rs : the time server using shared memory, a bizarre application

use std::ffi::CString;
use std::ptr;
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use libc::{shmat, shmget, shmdt, shmctl, IPC_CREAT, SHM_R, SHM_W, IPC_RMID};

const TIME_MEM_KEY: i32 = 99;       // kind of like a filename
const SEG_SIZE: usize = 100;        // size of segment

fn oops(message: &str, code: i32) {
    eprintln!("{}: {}", message, std::io::Error::last_os_error());
    std::process::exit(code);
}

fn main() {
    // create a shared memory segment
    let seg_id = unsafe { shmget(TIME_MEM_KEY, SEG_SIZE, IPC_CREAT | 0o777) };
    if seg_id == -1 {
        oops("shmget", 1);
    }

    // attach to it and get a pointer to where it attaches
    let mem_ptr = unsafe { shmat(seg_id, ptr::null(), 0) };
    if mem_ptr == libc::MAP_FAILED {
        oops("shmat", 2);
    }

    // run for a minute
    for _ in 0..60 {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");
        let c_time = CString::new(format!("{:?}", now)).expect("CString::new failed");
        unsafe {
            std::ptr::copy_nonoverlapping(c_time.as_ptr(), mem_ptr as *mut i8, c_time.as_bytes().len());
        }
        sleep(Duration::from_secs(1));
    }

    // now remove it
    unsafe {
        if shmctl(seg_id, IPC_RMID, ptr::null_mut()) == -1 {
            oops("shmctl", 3);
        }
    }
}
