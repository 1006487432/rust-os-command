use std::{env, fs};

fn main() {
    let mut args : Vec<String>= env::args().collect();
    match args.len(){
        1 => eprintln!("missing file operand"),
        2 => eprintln!("missing destination file operand after '{}'", args[1]),
        _ =>{
            match fs::copy(&args[1], &args[2]){
                Ok(len) =>{
                    println!("successfully copy {} bytes", len);
                },
                Err(e) =>{
                    eprintln!("{}",e);
                }
            };
        }
    };
}