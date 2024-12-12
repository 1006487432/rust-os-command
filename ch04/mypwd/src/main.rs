use std::env;
use std::fs::{self, read_dir};
use std::io;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::process::exit;

fn get_inode<P: AsRef<Path>>(path: P) -> io::Result<u64> {
    let metadata = fs::metadata(path)?;
    Ok(metadata.ino())
}

fn printpathto(this_inode: u64) {
    if let Ok(parent_inode) = get_inode("..") {
        if parent_inode != this_inode {
            if let Err(e) = env::set_current_dir("..") {
                eprintln!("Cannot change directory: {}", e);
                exit(1);
            }

            let mut its_name = String::new();
            if let Err(e) = inum_to_name(this_inode, &mut its_name) {
                eprintln!("Error: {}", e);
                exit(1);
            }

            let my_inode = get_inode(".").unwrap();
            printpathto(my_inode);
            print!("/{}", its_name);
        }
    }
}

fn inum_to_name(inode_to_find: u64, namebuf: &mut String) -> io::Result<()> {
    for entry in read_dir(".")? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        if metadata.ino() == inode_to_find {
            *namebuf = entry.file_name().into_string().unwrap();
            return Ok(());
        }
    }
    Err(io::Error::new(
        io::ErrorKind::NotFound,
        format!("Inode {} not found", inode_to_find),
    ))
}

fn main() {
    match get_inode(".") {
        Ok(inode) => {
            printpathto(inode);
            println!();
        }
        Err(e) => {
            eprintln!("Error getting inode: {}", e);
            exit(1);
        }
    }
}
