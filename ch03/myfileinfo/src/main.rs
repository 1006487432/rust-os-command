use std::env;
use std::fs;
use std::os::unix::fs::MetadataExt;
use std::time::{SystemTime, UNIX_EPOCH};

fn show_stat_info(filename: &str, metadata: &fs::Metadata) {
    println!("   mode: {:o}", metadata.mode());          // type + mode
    println!("  links: {}", metadata.nlink());           // # links
    println!("   user: {}", metadata.uid());             // user id
    println!("  group: {}", metadata.gid());             // group id
    println!("   size: {}", metadata.size());            // file size

    // Modified time
    if let Ok(duration) = metadata.modified()
        .unwrap()
        .duration_since(UNIX_EPOCH)
    {
        println!("modtime: {}", duration.as_secs());
    }

    println!("   name: {}", filename);                   // filename
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let filename = &args[1];
        match fs::metadata(filename) {
            Ok(metadata) => {
                show_stat_info(filename, &metadata);
                std::process::exit(0);
            }
            Err(e) => {
                eprintln!("Error: {}: {}", filename, e);
            }
        }
    } else {
        eprintln!("Usage: {} <filename>", args[0]);
    }

    std::process::exit(1);
}
