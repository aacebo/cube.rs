use std::{io, net};

use bytes::BufMut;

#[derive(Clone)]
pub struct ByteReader<T: io::Read> {
    inner: T,
}

impl<T: io::Read> ByteReader<T> {
    pub fn new(inner: T) -> Self {
        return Self { inner };
    }

    pub fn read<const N: usize>(&mut self) -> io::Result<[u8; N]> {
        let mut buf: [u8; N] = [0; N];
        self.read_at_most(&mut buf)?;
        return Ok(buf);
    }

    pub fn read_utf8<const N: usize>(&mut self) -> io::Result<String> {
        let bytes = self.read::<N>()?;
        return Ok(String::from_utf8_lossy(&bytes).to_string());
    }

    pub fn read_at_most(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut count = 0;

        loop {
            let size = self.inner.read(buf)?;
            count += size;

            if count == buf.len() || size == 0 {
                break;
            }
        }

        return Ok(count);
    }

    pub fn read_until<const N: usize>(&mut self, seq: &[u8; N]) -> io::Result<Vec<u8>> {
        let mut data = bytes::BytesMut::new();
        let mut buf = [0; 1];

        loop {
            let size = self.read_at_most(&mut buf)?;
            data.put_slice(&buf);

            if size == 0 {
                break;
            }

            if data.ends_with(seq) {
                data.truncate(data.len() - seq.len());
                break;
            }
        }

        return Ok(data.to_vec());
    }

    pub fn read_utf8_until<const N: usize>(&mut self, seq: &[u8; N]) -> io::Result<String> {
        let bytes = self.read_until(seq)?;
        return Ok(String::from_utf8_lossy(&bytes).to_string());
    }

    pub fn read_until_exclusive<const N: usize>(&mut self, seq: &[u8; N]) -> io::Result<Vec<u8>> {
        let mut data = bytes::BytesMut::new();
        let mut buf = [0; 1];

        loop {
            let size = self.read_at_most(&mut buf)?;
            data.put_slice(&buf);

            if size == 0 {
                break;
            }

            if data.ends_with(seq) {
                data.truncate(data.len() - seq.len());
                break;
            }
        }

        return Ok(data.to_vec());
    }

    pub fn read_utf8_until_exclusive<const N: usize>(
        &mut self,
        seq: &[u8; N],
    ) -> io::Result<String> {
        let bytes = self.read_until_exclusive(seq)?;
        return Ok(String::from_utf8_lossy(&bytes).to_string());
    }
}

impl ByteReader<&net::TcpStream> {
    pub fn peek<const N: usize>(&mut self) -> io::Result<[u8; N]> {
        let mut buf = [0; N];
        let mut count = 0;

        loop {
            let size = self.inner.peek(&mut buf)?;
            count += size;

            if count == N || size == 0 {
                break;
            }
        }

        return Ok(buf);
    }

    pub fn next_if<const N: usize>(&mut self, seq: &[u8; N]) -> bool {
        let buf = match self.peek::<N>() {
            Err(_) => return false,
            Ok(v) => v,
        };

        let is_match = buf.eq(seq);

        if is_match {
            let _ = self.read::<N>();
        }

        return is_match;
    }
}

impl<T: io::Read> io::Read for ByteReader<T> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        return self.inner.read(buf);
    }
}
