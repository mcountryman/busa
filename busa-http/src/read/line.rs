use std::io::Result;

use memchr::memchr;
use smol::io::AsyncReadExt;
use smol::prelude::*;

pub const BUFFER_SIZE: usize = 10_000;

pub struct LineReader<R: AsyncRead + Unpin>(R);

pub async fn read_line<R>(reader: &mut R) -> Result<Vec<u8>>
where
  R: AsyncRead + Unpin,
{
  let mut buf = vec![0u8; BUFFER_SIZE];
  let mut offset = 0;
  loop {
    let read = reader.read(&mut buf[offset..]).await?;
    if read == 0 {
      break;
    }

    if let Some(offset1) = memchr(b'\r', &buf[offset..]) {
      let offset = offset + offset1 + 1;
      if offset > buf.len() {
        break;
      }

      if buf[offset] == b'\n' {
        return Ok(Vec::from(&buf[..offset - 1]));
      }
    }

    offset += read;
    buf.resize(buf.len() + BUFFER_SIZE, 0);
  }

  Ok(Vec::new())
}

#[cfg(test)]
mod tests {
  use smol::io::Cursor;

  use crate::read::line::read_line;

  #[test]
  fn read_line_simple() {
    smol::block_on(async {
      let mut buf = b"GET * HTTP/1.1\r\n".to_vec();
      let mut buf = Cursor::new(&mut buf);
      let line = read_line(&mut buf).await.unwrap();

      assert_eq!(b"GET * HTTP/1.1".to_vec(), line);
    })
  }

  #[test]
  fn read_line_complex() {
    smol::block_on(async {
      let mut buf = b"GET https://www.w3.org/Protocols/HTTP/1.0/draft-ietf-http-spec.html#Request HTTP/1.1\r\n".to_vec();
      let mut buf = Cursor::new(&mut buf);
      let line = read_line(&mut buf).await.unwrap();

      assert_eq!(b"GET https://www.w3.org/Protocols/HTTP/1.0/draft-ietf-http-spec.html#Request HTTP/1.1".to_vec(), line);
    })
  }
}
