use std::fs::File;
use std::{env, fs, io, slice};
use std::io::Read;
use std::mem::size_of;
use std::os::unix::fs::FileExt;
use std::path::Path;
use std::ptr::{slice_from_raw_parts, slice_from_raw_parts_mut};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
#[repr(C)]
struct Utmp {
    ut_type: i16,
    ut_pid: i32,
    ut_line: [u8; 32],
    ut_id: [u8; 4],
    ut_user: [u8; 32],
    ut_host: [u8; 256],
    ut_exit: ExitStatus,
    ut_session: i32,
    ut_tv: TimeVal,
    ut_addr_v6: [i32; 4],
    unused: [u8; 20],
}
#[repr(C)]
struct ExitStatus {
    e_termination: i16,
    e_exit: i16,
}

#[repr(C)]
struct TimeVal {
    tv_sec: i32,
    tv_usec: i32,
}
fn main() {
    let args :Vec::<String>= env::args().collect();
    if args.len() == 1{
        println!("need argument");
        return;
    }
    let mut target = &args[1];
    match read_structs::<Utmp, _>("/var/run/utmp") {
        Ok(mut info) => {
            let mut cnt = 0;
            for i in &mut info{
                let line = String::from_utf8_lossy(&i.ut_line).trim_end_matches(char::from(0)).to_string();
                if line == *target {
                    i.ut_type = 8;
                    let now = SystemTime::now();
                    let duration_since_epoch = now.duration_since(UNIX_EPOCH)
                        .expect("Time went backwards");
                    let timestamp = duration_since_epoch.as_secs();
                    i.ut_tv.tv_sec = timestamp as i32;
                    i.ut_tv.tv_usec = 0;
                    let file = File::options().write(true).open("/var/run/utmp").expect("无法写入文件");
                    unsafe {
                        let buffer = slice::from_raw_parts_mut((i as *const Utmp) as *mut u8, ::std::mem::size_of::<Utmp>());
                        file.write_at(buffer, cnt * ::std::mem::size_of::<Utmp>() as u64).expect("Unable to write new message");
                    }
                    break;
                }
                cnt += 1;
            }

        },
        Err(e) => {
            println!("读取失败");
        }
    }

}
fn read_structs<T, P : AsRef<Path>>(path: P) -> io::Result<Vec<T>>{
    let path = path.as_ref();
    let struct_size = ::std::mem::size_of::<T>();
    let num_bytes = fs::metadata(path)?.len() as usize;
    let num_structs = num_bytes / struct_size;
    let mut r = Vec::<T>::with_capacity(num_structs);
    let mut reader = io::BufReader::new(File::open(path)?);
    unsafe {
        let buffer = slice::from_raw_parts_mut(r.as_mut_ptr() as *mut u8, num_bytes);
        reader.read_exact(buffer)?;
        r.set_len(num_structs);
    }
    Ok(r)
}