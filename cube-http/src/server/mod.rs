use std::{io, net};

pub mod router;

mod request;
use cube_url::Protocol;
pub use request::*;

mod response;
pub use response::*;

use crate::RequestMessage;

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

    fn on_connect(stream: net::TcpStream, _: net::SocketAddr) {
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
        let mut res = Response::<String>::new(stream);
        let response = res.protocol(&Protocol::from(message.protocol), &message.protocol_v);
        response.send().unwrap();
    }
}
