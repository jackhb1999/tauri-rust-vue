use std::io;

use mio::{Events, Interest, Poll, Token};
use mio::net::TcpListener;

const SERVER: Token = Token(0);

fn main() -> io::Result<()> {
    let addr = "127.0.0.1:8080".parse().unwrap();
    let mut listener = TcpListener::bind(addr)?;

    let mut events = Events::with_capacity(256);
    let mut poll = match Poll::new() {
        Ok(poll) => poll,
        Err(e) => panic!("failed to create Poll instance; err={:?}", e),
    };

    poll.registry().register(&mut listener, SERVER, Interest::READABLE | Interest::WRITABLE)?;
    loop {
        poll.poll(&mut events, None)?;
        for event in events.iter() {
            match event.token() {
                SERVER => loop {
                    let (mut tcpStream, address) = match listener.accept() {
                        Ok((tcpStream, adress)) => {
                            (tcpStream, adress)
                        }
                        Err(err) => {
                            return Err(err)
                        }
                    };
                    println!("Accepted connection from {}", address);
                },
                Token(_) => {}
            }
        }
    }
    return Ok(());
}
