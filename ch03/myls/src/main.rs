use std::ffi::{CString, OsStr};
use std::{fs, io};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{Duration, UNIX_EPOCH};
use users::{get_user_by_uid, get_current_uid, get_group_by_gid};
use ansi_term::Color::{Blue};
use libc::{S_IFBLK, S_IFCHR, S_IFDIR, S_IFMT, S_IRGRP, S_IROTH, S_IRUSR, S_IWGRP, S_IWOTH, S_IWUSR, S_IXGRP, S_IXOTH, S_IXUSR};
trait SetByIndex {
    fn set_by_index(&mut self, idx: usize, c: u8);
}
impl SetByIndex for String {
    fn set_by_index(&mut self, idx: usize, c: u8) {
        if idx>=self.len() {
            panic!("Index out of bounds: {}, expected: [0,{})", idx, self.len());
        }
        unsafe {
            let _buf: &mut [u8] = self.as_bytes_mut();
            _buf[idx] = c;
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        do_ls(&String::from('.'), &0);
    }else{
        let mut ops = 0;
        let mut p : Vec<String> = vec![];
        for path in &args[1..]{
            match path.as_str() {
                "-a" => ops |= 1,
                "-l" => ops |= 2,
                _ => if is_directory(path){
                    p.push((*path.clone()).parse().unwrap());
                }else{
                    println!("{} is not a directory", path);
                }
            }
        }
        let mut len = p.len();
        if len == 0{
            len += 1;
            p.push(String::from("."));
        }
        for path in p{
            if len > 1{
                println!("{}:", path);
            }
            do_ls(&path, &ops);
        }
    }
}
fn do_ls(path : &String, ops: &i32){
    let mut all : Vec<(String, bool, String)> = vec![];
    match fs::read_dir(path){
        Ok(dir) =>{
            for to in dir{
                let mut path: PathBuf = to.unwrap().path();
                match path.file_name() {
                    Some(name) => {
                        match name.to_str() {
                            Some(fin) => {
                                if (ops & 1 == 1)  ||fin.as_bytes()[0] != u8::try_from('.').unwrap(){
                                    all.push((fin.to_string(), path.is_dir(), path.to_string_lossy().to_string()));
                                }
                            },
                            _ => {}
                        }
                    },
                    _ => {}
                }

            }
        },
        Err(e) =>{
            eprintln!("{}", e);
        }
    }
    all.sort();
    for (idx, name) in all.iter().enumerate(){
        if ops & 2 == 2{
            show_stat_info(&name.2, &name.0)
        }else{
            if name.1 == true{
                print!("{:4} ", Blue.paint(&name.0));
            }else{
                print!("{:4} ", name.0);
            }
        }
        io::stdout().flush().unwrap();
    }
    println!();
}

fn is_directory(path: &String) -> bool {
    let path = Path::new(path);
    match fs::metadata(path) {
        Ok(metadata) => metadata.is_dir(),
        Err(_) => false,
    }
}

fn show_stat_info(path: &String, name: &String){
    unsafe {
        let root = CString::new((*path).clone()).unwrap();
        let mut info: libc::stat = std::mem::zeroed();
        if libc::stat(root.as_ptr(), &mut info) >= 0 {
            let modestr = mode_to_letters(info.st_mode);
            print!("{}", modestr);
            print!("{:4} ", info.st_nlink as i32);
            print!("{:8} ", get_user_by_uid(info.st_uid).unwrap().name().to_string_lossy());
            print!("{:8} ", get_group_by_gid(info.st_gid).unwrap().name().to_string_lossy());
            print!("{:8} ", info.st_size as i64);
            print!("{:8} ", format_time(info.st_mtime));
            print!("{}", name);
            println!();
        }
    }
}

fn mode_to_letters(mode: libc::mode_t) -> String {
    let mut str = String::from("----------");
    if mode & S_IFMT == S_IFDIR { str.set_by_index(0, 'd' as u8); }
    if mode & S_IFMT == S_IFCHR { str.set_by_index(0, 'c' as u8); }
    if mode & S_IFMT == S_IFBLK { str.set_by_index(0, 'b' as u8); }
    if mode & S_IRUSR != 0 { str.set_by_index(1, 'r' as u8); }
    if mode & S_IWUSR != 0 { str.set_by_index(2, 'w' as u8); }
    if mode & S_IXUSR != 0 { str.set_by_index(3, 'x' as u8); }
    if mode & S_IRGRP != 0 { str.set_by_index(4, 'r' as u8); }
    if mode & S_IWGRP != 0 { str.set_by_index(5, 'w' as u8); }
    if mode & S_IXGRP != 0 { str.set_by_index(6, 'x' as u8); }
    if mode & S_IROTH != 0 { str.set_by_index(7, 'r' as u8); }
    if mode & S_IWOTH != 0 { str.set_by_index(8, 'w' as u8); }
    if mode & S_IXOTH != 0 { str.set_by_index(9, 'x' as u8); }
    str
}

fn format_time(time: i64) -> String {
    let d = UNIX_EPOCH + Duration::new(time as u64, 0);
    let datetime = std::time::SystemTime::from(d);
    let datetime: chrono::DateTime<chrono::Local> = datetime.into();
    datetime.format("%b %e %H:%M").to_string()
}