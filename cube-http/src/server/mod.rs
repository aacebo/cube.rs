use std::{collections::HashMap, io, net};

pub mod router;

mod request;
pub use request::*;

use crate::{RequestMessage, ResponseMessage};

pub struct Server;

impl Server {
    pub fn run<A: net::ToSocketAddrs>(addr: A) -> io::Result<()> {
        let listener = net::TcpListener::bind(addr)?;

        loop {
            let (stream, addr) = listener.accept()?;
            let _ = std::thread::spawn(move || {
                Self::on_connect(stream, addr);
            });
        }
    }

    fn on_connect(mut stream: net::TcpStream, _: net::SocketAddr) {
        let message = match RequestMessage::read(&stream) {
            Err(err) => {
                println!("{}", err);
                return;
            }
            Ok(v) => v,
        };

        let request = match Request::<String>::try_from(&message) {
            Err(err) => {
                println!("{}", err);
                return;
            }
            Ok(v) => v,
        };

        println!("{}", request);

        let response = ResponseMessage {
            protocol: message.protocol,
            protocol_v: message.protocol_v,
            status: crate::Status::Ok,
            headers: HashMap::new(),
        };

        match response.write(&mut stream) {
            Err(err) => {
                println!("{}", err);
                return;
            }
            Ok(v) => v,
        };

        stream.shutdown(net::Shutdown::Both).unwrap();
    }
}
