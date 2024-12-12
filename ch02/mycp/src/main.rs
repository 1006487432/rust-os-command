use std::fs::File;
use std::io::{self, Read, Write};
use std::os::unix::fs::OpenOptionsExt;
use std::process::exit;
use std::os::unix::fs::PermissionsExt;
const BUFFERSIZE: usize = 4096;
const COPYMODE: u32 = 0o644;

fn oops(s1: &str, s2: &str) -> ! {
    eprintln!("Error: {} {}", s1, s2);
    exit(1);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        eprintln!("usage: {} source destination", args[0]);
        exit(1);
    }

    let src_path = &args[1];
    let dest_path = &args[2];

    let mut in_file = match File::open(src_path) {
        Ok(file) => file,
        Err(e) => oops("Cannot open", &format!("{}: {}", src_path, e)),
    };

    let mut out_file = match File::create(dest_path)
        .and_then(|f| f.set_permissions(std::fs::Permissions::from_mode(COPYMODE)).map(|_| f))
    {
        Ok(file) => file,
        Err(e) => oops("Cannot create", &format!("{}: {}", dest_path, e)),
    };

    let mut buf = vec![0; BUFFERSIZE];
    loop {
        match in_file.read(&mut buf) {
            Ok(0) => break, // Reached EOF
            Ok(n) => {
                if out_file.write_all(&buf[..n]).is_err() {
                    oops("Write error to", dest_path);
                }
            }
            Err(e) => oops("Read error from", &format!("{}: {}", src_path, e)),
        }
    }

    if let Err(e) = in_file.sync_all() {
        oops("Error syncing input file", &format!("{}: {}", src_path, e));
    }

    if let Err(e) = out_file.sync_all() {
        oops("Error syncing output file", &format!("{}: {}", dest_path, e));
    }
}
