use std::{
    io::{Read, Write},
    net::TcpStream,
    os::fd::{AsFd, AsRawFd, FromRawFd},
};
use tsnet::TSNet;

fn main() -> Result<(), String> {
    let config = tsnet::ConfigBuilder::new().ephemeral(true).build()?;

    let mut ts = TSNet::new(config)?;
    ts.up()?;

    let listener = ts.listen("tcp", ":1999")?;
    let listener_fd = listener.as_fd();

    loop {
        let conn = ts.accept(listener_fd).unwrap();
        let conn_fd = conn.as_fd();
        let mut stream = unsafe { TcpStream::from_raw_fd(conn.as_raw_fd()) };
        let remote_addr = ts.get_remote_addr(conn_fd, listener_fd).unwrap();
        let mut buf = [0; 1024];

        println!("connection from: {}", remote_addr);

        while let Ok(n) = stream.read(&mut buf) {
            if n == 0 {
                break;
            }
            stream.write_all(&buf[..n]).unwrap();
            stream.flush().unwrap();
        }

        return Ok(());
    }
}
