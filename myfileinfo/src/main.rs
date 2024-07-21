use std::ffi::CString;

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args.len() == 1{
        eprintln!("Usage: {} <filename>", args[0]);
        return;
    }
    unsafe {
        let root = CString::new(args[1].clone()).unwrap();
        let mut info: libc::stat = std::mem::zeroed();
        if libc::stat(root.as_ptr(), &mut info) >= 0 {
            println!("    mode: {}", info.st_mode);
            println!("   links: {}", info.st_nlink);
            println!("    user: {}", info.st_uid);
            println!("   group: {}", info.st_gid);
            println!("    size: {}", info.st_size);
            println!(" modtime: {}", info.st_mtime);
            println!("    name: {}", args[1]);
        }
    }
}