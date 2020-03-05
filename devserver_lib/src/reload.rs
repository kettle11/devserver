extern crate base64;
extern crate notify;

use sha1::{Digest, Sha1};
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::Path;
use std::str;
use std::thread;

pub const RELOAD_PORT: u32 = 8129; /* Arbitrary port */

fn parse_websocket_handshake(bytes: &[u8]) -> String {
    let request_string = str::from_utf8(&bytes).unwrap();
    let lines = request_string.split("\r\n");
    let mut sec_websocket_key = "";

    for line in lines {
        let parts: Vec<&str> = line.split(':').collect();
        if let "Sec-WebSocket-Key" = parts[0] {
            sec_websocket_key = parts[1].trim();
        }
    }

    // Perform a ceremony of getting the SHA1 hash of the sec_websocket_key joined with
    // an arbitrary string and then take the base 64 encoding of that.
    let sec_websocket_accept = format!(
        "{}{}",
        sec_websocket_key, "258EAFA5-E914-47DA-95CA-C5AB0DC85B11"
    );
    let mut hasher = Sha1::new();
    hasher.input(sec_websocket_accept.as_bytes());
    let result = hasher.result();
    let bytes = base64::encode(&result);

    format!("HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: {}\r\n\r\n",bytes)
}

// This function can send strings of text to a websocket stream.
fn send_websocket_message<T: Write>(mut stream: T, message: &str) -> Result<(), std::io::Error> {
    let message_bytes = message.as_bytes();
    let payload_length = message_bytes.len();

    stream.write_all(&[129])?; // Devserver always sends text messages. The combination of bitflags and opcode produces '129'
    let mut second_byte: u8 = 0;
    if payload_length < 125 {
        second_byte |= payload_length as u8;
        stream.write_all(&[second_byte])?;
    } else if payload_length < std::u16::MAX as usize {
        stream.write_all(&((126 + payload_length) as u16).to_be_bytes())?; // Write the length as a u16
    } else if payload_length < std::u64::MAX as usize {
        stream.write_all(&((127 + payload_length) as u16).to_be_bytes())?; // Write the length as a u64
    } else {
        println!("Message too large");
    }

    stream.write_all(&message_bytes)?;
    Ok(())
}

fn handle_websocket_handshake<T: Read + Write>(mut stream: T) {
    let header = crate::read_header(&mut stream);
    let response = parse_websocket_handshake(&header);
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

pub fn watch_for_reloads(address: &str, path: &str) {
    // Setup websocket receiver.
    let websocket_address = format!("{}:{:?}", address, RELOAD_PORT);
    let listener = TcpListener::bind(websocket_address).unwrap();

    // The only incoming message we expect to receive is the initial handshake.
    for stream in listener.incoming() {
        let path = path.to_owned();

        thread::spawn(move || {
            if let Ok(mut stream) = stream {
                handle_websocket_handshake(&mut stream);

                // We do not handle ping/pong requests. Is that bad?
                // This code also assumes the client will never send any messages
                // other than the initial handshake.
                let (tx, rx) = std::sync::mpsc::channel();
                use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
                /* Is a 10ms delay here too short?*/
                let mut watcher: RecommendedWatcher =
                    Watcher::new(tx, std::time::Duration::from_millis(10)).unwrap();
                watcher
                    .watch(Path::new(&path), RecursiveMode::Recursive)
                    .unwrap();

                // Watch for file changes until the socket closes.
                loop {
                    match rx.recv() {
                        Ok(event) => {
                            let (_path, refresh) = match event {
                                DebouncedEvent::NoticeWrite(path) => (path, false),
                                DebouncedEvent::NoticeRemove(path) => (path, false),
                                DebouncedEvent::Create(path) => (path, true),
                                DebouncedEvent::Write(path) => (path, true),
                                DebouncedEvent::Chmod(path) => (path, true),
                                DebouncedEvent::Remove(path) => (path, true),
                                DebouncedEvent::Rename(old_path, _new_path) => (old_path, false),
                                DebouncedEvent::Rescan => (std::path::PathBuf::new(), false),
                                DebouncedEvent::Error(..) => panic!(),
                            };

                            if refresh {
                                // A blank message is sent triggering a refresh on any file change.
                                // In the future a filepath be sent here.
                                // If this message fails to send, then likely the socket has been closed.
                                if send_websocket_message(&stream, "").is_err() {
                                    break;
                                };
                            }
                        }
                        Err(e) => println!("File watch error: {:?}", e),
                    };
                }
            }
        });
    }
}
