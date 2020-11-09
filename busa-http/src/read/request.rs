use std::error::Error;
use std::fmt::{Display, Formatter};

use memchr::memchr_iter;
use smol::prelude::AsyncRead;

use crate::read::line::read_line;
use crate::{HttpMethod, HttpVersion};

type ReadRequestResult<T> = Result<T, ReadRequestError>;

#[derive(Debug)]
pub enum ReadRequestError {
  Io(std::io::Error),
  BadMethod,
  BadRequest,
  BadVersion,
}

pub async fn read_request<R>(
  reader: &mut R,
) -> ReadRequestResult<(HttpMethod, String, HttpVersion)>
where
  R: AsyncRead + Unpin,
{
  let line = read_line(reader).await?;
  let parts: Vec<usize> = memchr_iter(b' ', &line).collect();
  if parts.len() < 2 {
    return Err(ReadRequestError::BadRequest);
  }

  let method = &line[..parts[0]];
  let method = match method {
    b"GET" => Ok(HttpMethod::Get),
    b"PUT" => Ok(HttpMethod::Put),
    b"POST" => Ok(HttpMethod::Post),
    b"HEAD" => Ok(HttpMethod::Head),
    b"TRACE" => Ok(HttpMethod::Trace),
    b"CONNECT" => Ok(HttpMethod::Connect),
    b"OPTIONS" => Ok(HttpMethod::Options),
    _ => Err(ReadRequestError::BadMethod),
  }?;

  let uri = &line[parts[0] + 1..parts[1]];
  let uri = String::from_utf8(uri.to_vec()).map_err(|_| ReadRequestError::BadRequest)?;

  let version = &line[parts[1] + 1..];
  let version = match version {
    b"HTTP/1.0" => Ok(HttpVersion::V1_0),
    b"HTTP/1.1" => Ok(HttpVersion::V1_1),
    _ => Err(ReadRequestError::BadVersion),
  }?;

  Ok((method, uri, version))
}

impl From<std::io::Error> for ReadRequestError {
  fn from(err: std::io::Error) -> Self {
    ReadRequestError::Io(err)
  }
}

impl Display for ReadRequestError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl Error for ReadRequestError {}

#[cfg(test)]
mod tests {
  use smol::io::Cursor;

  use super::read_request;
  use crate::{HttpMethod, HttpVersion};

  #[test]
  fn read_request_simple() {
    smol::block_on(async {
      let mut buf = b"GET * HTTP/1.1\r\n".to_vec();
      let mut buf = Cursor::new(&mut buf);
      let (method, uri, version) = read_request(&mut buf).await.unwrap();

      assert_eq!(method, HttpMethod::Get);
      assert_eq!(uri, "*");
      assert_eq!(version, HttpVersion::V1_1);
    })
  }

  #[test]
  fn read_request_complex() {
    smol::block_on(async {
      let mut buf =
        b"PUT https://www.w3.org/Protocols/HTTP/1.0/draft-ietf-http-spec.html#Request\
       HTTP/1.0\r\n"
          .to_vec();
      let mut buf = Cursor::new(&mut buf);
      let (method, uri, version) = read_request(&mut buf).await.unwrap();

      assert_eq!(method, HttpMethod::Put);
      assert_eq!(
        uri,
        "https://www.w3.org/Protocols/HTTP/1.0/draft-ietf-http-spec.html#Request"
      );
      assert_eq!(version, HttpVersion::V1_0);
    })
  }
}
