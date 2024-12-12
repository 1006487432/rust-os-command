// shm_ts2.rs - time server shared mem ver2 : use semaphores for locking

use std::ffi::CString;
use std::ptr;
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use libc::{shmat, shmget, shmctl, semget, semctl, semop, IPC_CREAT, IPC_EXCL, IPC_RMID, sembuf, SIGINT, signal};

const TIME_MEM_KEY: i32 = 99;       // like a filename
const TIME_SEM_KEY: i32 = 9900;
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

static mut SEG_ID: libc::c_int = 0;
static mut SEMSET_ID: libc::c_int = 0;

fn oops(message: &str, code: i32) {
    eprintln!("{}: {}", message, std::io::Error::last_os_error());
    std::process::exit(code);
}

extern "C" fn cleanup(_sig: libc::c_int) {
    unsafe {
        shmctl(SEG_ID, IPC_RMID, std::ptr::null_mut::<libc::shmid_ds>());
        semctl(SEMSET_ID, 0, IPC_RMID, std::ptr::null_mut::<libc::semid_ds>());
    }
    std::process::exit(0);
}

fn set_sem_value(semset_id: libc::c_int, semnum: libc::c_int, val: libc::c_int) {
    let sem_union = UnionSemun { val };
    unsafe {
        if semctl(semset_id, semnum, SETVAL, sem_union) == -1 {
            oops("semctl", 4);
        }
    }
}

fn wait_and_lock(semset_id: libc::c_int) {
    let actions = [
        sembuf { sem_num: 0, sem_op: 0, sem_flg: SEM_UNDO }, // wait until no readers
        sembuf { sem_num: 1, sem_op: 1, sem_flg: SEM_UNDO }, // increment num writers
    ];

    if unsafe { semop(semset_id, actions.as_ptr() as *mut sembuf, actions.len() as libc::size_t) } == -1 {
        oops("semop: locking", 10);
    }
}

fn release_lock(semset_id: libc::c_int) {
    let actions = [sembuf { sem_num: 1, sem_op: -1, sem_flg: SEM_UNDO }]; // decrement num writers

    if unsafe { semop(semset_id, actions.as_ptr() as *mut sembuf, actions.len() as libc::size_t) } == -1 {
        oops("semop: unlocking", 10);
    }
}

fn main() {
    unsafe {
        SEG_ID = shmget(TIME_MEM_KEY, SEG_SIZE, IPC_CREAT | 0o777);
        if SEG_ID == -1 {
            oops("shmget", 1);
        }

        let mem_ptr = shmat(SEG_ID, ptr::null(), 0);
        if mem_ptr == libc::MAP_FAILED {
            oops("shmat", 2);
        }

        SEMSET_ID = semget(TIME_SEM_KEY, 2, 0o666 | IPC_CREAT | IPC_EXCL);
        if SEMSET_ID == -1 {
            oops("semget", 3);
        }

        set_sem_value(SEMSET_ID, 0, 0); // set counters
        set_sem_value(SEMSET_ID, 1, 0); // both to zero

        signal(SIGINT, cleanup as usize);

        for _ in 0..60 {
            let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");
            let c_time = CString::new(format!("{:?}", now)).expect("CString::new failed");

            println!("\tshm_ts2 waiting for lock");
            wait_and_lock(SEMSET_ID);
            println!("\tshm_ts2 updating memory");

            std::ptr::copy_nonoverlapping(c_time.as_ptr(), mem_ptr as *mut i8, c_time.as_bytes().len());
            sleep(Duration::from_secs(5));

            release_lock(SEMSET_ID);
            println!("\tshm_ts2 released lock");
            sleep(Duration::from_secs(1));
        }

        cleanup(0);
    }
}
