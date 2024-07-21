use std::fs::File;
use std::{fs, io, slice};
use std::io::Read;
use std::mem::size_of;
use std::path::Path;
use std::ptr::{slice_from_raw_parts, slice_from_raw_parts_mut};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use chrono::{DateTime, NaiveDate, NaiveDateTime};

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
    match read_structs::<Utmp, _>("/var/run/utmp") {
        Ok(info) => {
            for i in &info{
                show_info(i);
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

fn show_info(utmp: &Utmp) {
    if utmp.ut_type != 7 { return }
    let user = String::from_utf8_lossy(&utmp.ut_user).trim_end_matches(char::from(0)).to_string();
    let line = String::from_utf8_lossy(&utmp.ut_line).trim_end_matches(char::from(0)).to_string();
    let login_time = DateTime::from_timestamp(utmp.ut_tv.tv_sec as i64, 0).unwrap(); //日期和unix时间戳得转换
    let host = String::from_utf8_lossy(&utmp.ut_host).trim_end_matches(char::from(0)).to_string();
    println!("{:8} {:8} {} {:?} ({})", user, line, utmp.ut_type, login_time, host);
}
