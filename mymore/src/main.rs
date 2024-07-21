use std::{env, fs};
use std::io;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, stdin, Write};
use crossterm::{cursor, event::{self, Event, KeyEvent, KeyEventKind}, execute};
use crossterm::event::{KeyCode, KeyModifiers};
use crossterm::event::KeyCode::F;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

const PAGELEN: i32 = 24;
const LINELEN: i32 = 512;
fn main() {
    let args : Vec<String> = env::args().collect();
    let argc = args.len();
    if argc == 1 {
        //从标准流中读取
        do_more(io::stdin().lock(), 0);
    }else{
        //从文件中读取
        for path in &args[1..]{
            let file = io::BufReader::new(File::open(path).expect("无法打开文件"));
            let all = fs::metadata(path).expect("无法打开文件").len() as usize;
            do_more(file, all);
        }
    }
}
// 接受一个实现BufRead的泛型R
fn do_more<R: BufRead>(mut reader: R, all: usize){
    let mut num_of_lines:i32 = 0;
    let mut cmd = io::BufReader::new(File::open("/dev/tty").expect("无法打开 /dev/tty"));
    let reader = io::BufReader::with_capacity(LINELEN as usize, reader).lines();
    let mut have:usize = 0;
    let mut input = 0;
    for line in reader{
        if num_of_lines == PAGELEN{
            //let reply = see_more(&mut cmd);
            let reply = see_more_better(&have);
            if reply == 0{
                break
            }
            num_of_lines -= reply;
        }
        if let Ok(text) = line{
            println!("{}",text);
            input += text.len() + 1;
            if all != 0{
                have = input * 100 / all;
            }
            num_of_lines += 1;
        }
    }
}
fn see_more(cmd: &mut BufReader<File>) -> i32{
    println!("\x1B[7m more? \x1B[m");
    let mut input = String::new();
    cmd.read_line(&mut input).unwrap();
    let c = input.chars().next().unwrap();
    match c {
        'q' => 0,
        ' ' => PAGELEN ,
        '\n' => 1,
        _ => 0
    }
}
fn get_key() -> Option<KeyEvent> {
    if let Ok(Event::Key(key)) = event::read() {
        // 对按键事件进行判断是为了防止一次按键的点击触发多次事件
        // 所以对其限制为只有按键被按下时返回 Some 值。
        if key.kind == KeyEventKind::Press {
            return Some(key);
        }
    }
    None
}
fn see_more_better(have: &usize) -> i32 {
    match *have{
        0 => print!("\x1B[7m --更多-- \x1B[m"),
        _ => print!("\x1B[7m --更多-- %{} \x1B[m", have)

    }
    io::stdout().flush().expect("TODO: panic message");
    enable_raw_mode().unwrap();
    let mut flag = 0;
    loop {
        let Some(key) = get_key() else{
            continue;
        };
        if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c') {
            break;
        }
        match key.code {
            KeyCode::Enter => flag = 1,
            KeyCode::Char(ch) => {
                match ch {
                    'q' => flag = 0,
                    ' ' => flag = PAGELEN ,
                    '\n' => flag= 1,
                    _ => flag = 0
                }
            }
            _ => flag = 0
        }
        break;
    }
    disable_raw_mode().unwrap();
    execute!(io::stdout(), cursor::MoveToColumn(0)).expect("error");
    // 清除光标后的字符
    const BACKSPACE: &'static str = "\x1B[K";
    print!("{}", BACKSPACE);
    flag
}