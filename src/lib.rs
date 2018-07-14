use std::net::{ToSocketAddrs, TcpStream};
use std::io;
use std::io::{Read, Write, BufReader, BufRead, ErrorKind};

#[macro_use]
extern crate structure;

use std::time::Duration;

pub fn topic<A: ToSocketAddrs>(addr : A, msg : &str, timeout : usize) -> io::Result<String> {
    let mut stream = TcpStream::connect_timeout(&addr.to_socket_addrs()?.next().expect("No addr found"), Duration::from_millis(timeout as u64))?;

    let mut request : Vec<u8> = Vec::with_capacity(msg.len() + 10);
    request.append(&mut generate_header(msg.len() as u8));
    request.extend_from_slice(msg.as_bytes());
    request.push(0x00);

    stream.write(&request)?;

    let resp_header_struct = structure!("HHx");
    let (magic_num, resp_size) = resp_header_struct.unpack_from(&mut stream)?;

    if magic_num != 0x0083 {
        return Err(io::Error::new(ErrorKind::Other, "Wrong magic number in byond header"));
    }

    let resp_size = resp_size as usize;

    let mut resp_buffer = Vec::with_capacity(resp_size - 1);
    let read_bytes = BufReader::new(stream).take(resp_size as u64).read_until(0, &mut resp_buffer)?;

    if read_bytes != resp_size - 1 {
        return Err(io::Error::new(ErrorKind::Other, "Actual read did not match byond header"));
    }

    Ok(String::from_utf8(resp_buffer).unwrap())
}

fn generate_header(size : u8) -> Vec<u8> {
    vec![0x00, 0x83, 0x00, size + 6, 0, 0, 0, 0, 0]
}