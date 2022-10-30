use std::convert::Infallible;
use std::net::SocketAddr;

use bytes::Bytes;
use http_body_util::Full;
use hyper::service::service_fn;
use hyper::{Request, Response};
use tokio::net::TcpListener;

use super::common::NetError;

pub struct Server {

}

impl Server {
    fn start() -> Result<(), NetError> {

    }
}
