use std::env;

fn main() {
    let current_dir = env::current_dir().expect("无法读取目录");
    println!("{}", current_dir.to_string_lossy().to_string());
}
