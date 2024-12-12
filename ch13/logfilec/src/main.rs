use std::env;
use std::io;
use std::os::unix::net::UnixDatagram;
use std::path::Path;

const SOCKET_PATH: &str = "/tmp/logfilesock";

fn oops(message: &str, code: i32) -> ! {
    eprintln!("{}", message);
    std::process::exit(code);
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: logfilec 'message'");
        std::process::exit(1);
    }
    let msg = &args[1];

    let socket = UnixDatagram::unbound().map_err(|e| {
        eprintln!("socket error: {}", e);
        e
    })?;

    let sock_path = Path::new(SOCKET_PATH);

    socket.send_to(msg.as_bytes(), &sock_path).map_err(|e| {
        eprintln!("sendto error: {}", e);
        e
    })?;

    Ok(())
}
