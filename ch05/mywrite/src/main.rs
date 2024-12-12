use std::fs::File;
use std::io;
use std::io::Write;

fn main() {
    let args :Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: mywrite ttyname");
        return;
    }
    let mut target = File::options().append(true).open(&args[1]).expect("无法写入文件");
    let mut input = String::new();
    loop {
        match io::stdin().read_line(&mut input) {
            Ok(0) => break,
            Err(e) => eprintln!("{}", e),
            _ => {
                target.write((&input).as_ref()).expect("写入文件失败");
                input.clear();
            }
        }
    }
}
