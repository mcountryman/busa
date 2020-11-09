pub mod headers;
pub mod read;
pub mod request;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum HttpVersion {
  V1_0,
  V1_1,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum HttpStatusCode {
  Okay = 200,
  Created = 201,
  NoContent = 202,
  MovedPermanently = 301,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum HttpMethod {
  Get,
  Put,
  Post,
  Head,
  Trace,
  Connect,
  Options,
}
