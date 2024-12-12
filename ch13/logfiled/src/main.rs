use std::ffi::CString;
use std::os::unix::net::UnixDatagram;
use std::time::{SystemTime, UNIX_EPOCH};
use std::io::Error;

const MSGLEN: usize = 512;
const SOCKNAME: &str = "/tmp/logfilesock";

fn main() -> Result<(), Error> {
    let sock = UnixDatagram::bind(SOCKNAME)?;
    let mut msg = [0u8; MSGLEN];
    let mut msgnum = 0;

    loop {
        let (len, _addr) = sock.recv_from(&mut msg)?;
        let received_msg = std::str::from_utf8(&msg[..len]).unwrap_or("");

        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let timestr = format!("{:?}", now);

        println!("[{:5}] {} {}", msgnum, timestr, received_msg);
        msgnum += 1;
    }
}
