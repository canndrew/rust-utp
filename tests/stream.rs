#![feature(old_io)]

extern crate utp;

use std::thread;
use utp::UtpStream;
use std::io::{Read, Write};

macro_rules! iotry {
    ($e:expr) => (match $e { Ok(e) => e, Err(e) => panic!("{}", e) })
}

fn next_test_ip4<'a>() -> (&'a str, u16) {
    use std::old_io::test::next_test_port;
    ("127.0.0.1", next_test_port())
}

#[test]
fn test_stream_open_and_close() {
    let server_addr = next_test_ip4();
    let mut server = iotry!(UtpStream::bind(server_addr));

    thread::spawn(move || {
        let mut client = iotry!(UtpStream::connect(server_addr));
        iotry!(client.close());
        drop(client);
    });

    let mut received = vec!();
    iotry!(server.read_to_end(&mut received));
    iotry!(server.close());
}

#[test]
fn test_stream_small_data() {
    // Fits in a packet
    const LEN: usize = 1024;
    let data: Vec<u8> = (0..LEN).map(|idx| idx as u8).collect();
    assert_eq!(LEN, data.len());

    let d = data.clone();
    let server_addr = next_test_ip4();
    let mut server = iotry!(UtpStream::bind(server_addr));

    thread::spawn(move || {
        let mut client = iotry!(UtpStream::connect(server_addr));
        iotry!(client.write(&d[..]));
        iotry!(client.close());
    });

    let mut received = vec!();
    iotry!(server.read_to_end(&mut received));
    assert!(!received.is_empty());
    assert_eq!(received.len(), data.len());
    assert_eq!(received, data);
}

#[test]
fn test_stream_large_data() {
    // Has to be sent over several packets
    const LEN: usize = 1024 * 1024;
    let data: Vec<u8> = (0..LEN).map(|idx| idx as u8).collect();
    assert_eq!(LEN, data.len());

    let d = data.clone();
    let server_addr = next_test_ip4();
    let mut server = iotry!(UtpStream::bind(server_addr));

    thread::spawn(move || {
        let mut client = iotry!(UtpStream::connect(server_addr));
        iotry!(client.write(&d[..]));
        iotry!(client.close());
    });

    let mut received = vec!();
    iotry!(server.read_to_end(&mut received));
    assert!(!received.is_empty());
    assert_eq!(received.len(), data.len());
    assert_eq!(received, data);
}

#[test]
#[ignore]
fn test_stream_successive_reads() {
    const LEN: usize = 1024;
    let data: Vec<u8> = (0..LEN).map(|idx| idx as u8).collect();
    assert_eq!(LEN, data.len());

    let d = data.clone();
    let server_addr = next_test_ip4();
    let mut server = iotry!(UtpStream::bind(server_addr));

    thread::spawn(move || {
        let mut client = iotry!(UtpStream::connect(server_addr));
        iotry!(client.write(&d[..]));
        iotry!(client.close());
    });

    let mut received = vec!();
    iotry!(server.read_to_end(&mut received));

    let mut buf = [0u8; 4096];
    match server.read(&mut buf) {
        Err(_) => {}, // Closed
        e => panic!("should have failed with Closed, got {:?}", e),
    };
}
