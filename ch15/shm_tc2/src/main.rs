// shm_tc2.rs - time client shared mem ver2 : use semaphores for locking

use std::ffi::CStr;
use std::ptr;
use libc::{shmat, shmget, shmdt, semget, semop, IPC_CREAT, SHM_R, SHM_W, sembuf, IPC_EXCL, IPC_RMID};

const TIME_MEM_KEY: i32 = 99;       // kind of like a port number
const TIME_SEM_KEY: i32 = 9900;     // like a filename
const SEG_SIZE: usize = 100;        // size of segment
const SEM_UNDO: i16 = 0x1000;       // manually define SEM_UNDO
const SETVAL: i32 = 16;             // manually define SETVAL

// Define the union semun
#[repr(C)]
union UnionSemun {
    val: libc::c_int,
    buf: *mut libc::semid_ds,
    array: *mut libc::c_ushort,
}

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
    if mem_ptr == libc::MAP_FAILED {
        oops("shmat", 2);
    }

    // create a semaphore set with 2 semaphores
    let semset_id = unsafe { semget(TIME_SEM_KEY, 2, IPC_CREAT | IPC_EXCL | 0o666) };
    if semset_id == -1 {
        oops("semget", 3);
    }

    // initialize semaphore values
    let sem_union_val1 = UnionSemun { val: 1 };
    unsafe {
        if libc::semctl(semset_id, 0, SETVAL, sem_union_val1) == -1 {
            oops("semctl", 4);
        }
    }
    let sem_union_val0 = UnionSemun { val: 0 };
    unsafe {
        if libc::semctl(semset_id, 1, SETVAL, sem_union_val0) == -1 {
            oops("semctl", 5);
        }
    }

    wait_and_lock(semset_id);

    unsafe {
        let c_str = CStr::from_ptr(mem_ptr as *const _);
        if let Ok(str_slice) = c_str.to_str() {
            println!("The time, direct from memory: ..{}", str_slice);
        } else {
            oops("Reading shared memory", 6);
        }
    }

    release_lock(semset_id);
    unsafe { shmdt(mem_ptr) };

    // remove the semaphore set
    let sem_union_rmid = UnionSemun { val: 0 };
    unsafe {
        if libc::semctl(semset_id, 0, IPC_RMID, sem_union_rmid) == -1 {
            oops("semctl", 7);
        }
    }
}

fn wait_and_lock(semset_id: i32) {
    let actions = [
        sembuf { sem_num: 1, sem_op: 0, sem_flg: SEM_UNDO }, // wait for 0 on n_writers
        sembuf { sem_num: 0, sem_op: 1, sem_flg: SEM_UNDO }, // increment n_readers
    ];

    if unsafe { semop(semset_id, actions.as_ptr() as *mut sembuf, actions.len() as libc::size_t) } == -1 {
        oops("semop: locking", 10);
    }
}

fn release_lock(semset_id: i32) {
    let actions = [sembuf { sem_num: 0, sem_op: -1, sem_flg: SEM_UNDO }]; // decrement num_readers

    if unsafe { semop(semset_id, actions.as_ptr() as *mut sembuf, actions.len() as libc::size_t) } == -1 {
        oops("semop: unlocking", 10);
    }
}
