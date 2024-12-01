use std::collections::HashMap;
use std::fmt::format;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use mio::{Events, Interest, Poll, Token};
use mio::net::TcpListener;

const SERVER: Token = Token(0);

fn next(token: &mut Token) -> Token {
    let token = Token(token.0 + 1);
    return token;
}

fn main() -> io::Result<()> {
    let addr = "127.0.0.1:8080".parse().unwrap();
    let mut listener = TcpListener::bind(addr)?;

    let mut events = Events::with_capacity(256);
    let mut poll = match Poll::new() {
        Ok(poll) => poll,
        Err(e) => panic!("failed to create Poll instance; err={:?}", e),
    };

    let mut cur_token = Token(SERVER.0 + 1);

    poll.registry().register(&mut listener, SERVER, Interest::READABLE | Interest::WRITABLE)?;

    let mut hash_map = HashMap::new();
    // let mut string = String::new();
    let mut message = HashMap::new();
    loop {
        poll.poll(&mut events, None)?;
        for event in events.iter() {
            match event.token() {
                SERVER => loop {
                    let (mut tcpStream, address) = match listener.accept() {
                        Ok((tcpStream, adress)) => {
                            (tcpStream, adress)
                        }
                        Err(ref err) if err.kind() == io::ErrorKind::WouldBlock => break,
                        Err(err) => {
                            return Err(err)
                        }
                    };
                    println!("Accepted connection from {}", address);
                    let token = next(&mut cur_token);
                    tcpStream.write_all(b"hello world\r\n").unwrap();
                    poll.registry().register(&mut tcpStream, token, Interest::READABLE | Interest::WRITABLE)?;
                    hash_map.insert(token, tcpStream);
                    message.insert(token, String::new());
                },
                token => {
                    let mut flag = false;
                    let tcp_stream = hash_map.get_mut(&token).unwrap();
                    let mut buffer_read = BufReader::new(&mut *tcp_stream);
                    let mut lineString = message.get_mut(&token).unwrap();
                    if event.is_readable() {
                        loop {
                            match buffer_read.read_line(&mut lineString) {
                                Ok(_) if lineString == "\r\n" => {
                                    flag = true;
                                    break;
                                }
                                Ok(_) => break,
                                Err(ref err) if err.kind() == ErrorKind::WouldBlock => break,
                                Err(err) if err.kind() == ErrorKind::ConnectionReset => {
                                    flag = true;
                                    break;
                                }
                                Err(err) => { return Err(err) }
                            }
                        }
                        if lineString.len() > 2 && &lineString[lineString.len() - 2..] == "\r\n" {
                            buffer_read.get_mut().write_all(lineString.as_bytes())?;
                            poll.registry().reregister(*buffer_read.get_mut(), token, Interest::READABLE)?;
                            lineString.clear();
                        }
                    }
                    if flag {
                        poll.registry().deregister(hash_map.get_mut(&token).unwrap())?;
                        hash_map.remove(&token);
                        message.remove(&token);
                    }
                }
            }
        }
    }
    return Ok(());
}
