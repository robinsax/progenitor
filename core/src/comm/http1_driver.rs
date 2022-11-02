use std::sync::Arc;
use std::net::{SocketAddr, AddrParseError};

use async_trait::async_trait;
use bytes::Bytes;
use tokio::net::TcpListener;
use http_body_util::{Full, BodyExt};

// TODO: Likely a temporary dependency.
use hyper::{service::service_fn, server::conn::http1::Builder};

use crate::EnvConfig;
use crate::env_config::FromEnvConfig;
use crate::errors::InitError;

use super::errors::CommError;
use super::server::Server;
use super::driver::CommunicationDriver;
use super::io::{Request, Response, Route};

impl From<AddrParseError> for InitError {
    fn from(err: AddrParseError) -> Self {
        Self::NotImplemented(format!("<addr_parse {:?}>", err))
    }
}

impl From<tokio::io::Error> for CommError {
    fn from(err: tokio::io::Error) -> Self {
        Self::NotImplemented(format!("<tokio::io {:?}>", err))
    }
}

impl From<hyper::Error> for CommError {
    fn from(err: hyper::Error) -> Self {
        Self::NotImplemented(format!("<hyper:: {:?}>", err))
    }
}

// TODO so bad
async fn prep_request(hyper_req: hyper::Request<hyper::body::Incoming>) -> Result<Request, CommError> {
    let path = hyper_req.uri().clone().to_string();
    let raw = hyper_req.collect().await?.to_bytes();

    Ok(Request { route: Route::new(path), payload: raw.into() })
}

async fn prep_response(resp: Response) -> hyper::Response<Full<Bytes>> {
    hyper::Response::new(
        match resp.payload.try_into_bytes() {
            Ok(bytes) => Full::new(bytes),
            Err(_) => {
                todo!("TODO: Response error handling");
            }
        }
    )
}

pub struct Http1CommunicationDriver {
    socker_addr: SocketAddr
}

impl FromEnvConfig for Http1CommunicationDriver {
    fn try_from_config(_: EnvConfig) -> Result<Self, InitError> {
        Ok(Http1CommunicationDriver {
            socker_addr: "TODO".parse()?
        })
    }
}

#[async_trait]
impl CommunicationDriver for Http1CommunicationDriver {
    async fn handle_connections(self, server: Arc<Server<Http1CommunicationDriver>>) -> Result<(), CommError> {
        let listener = TcpListener::bind(self.socker_addr).await?;

        loop {
            let (stream, _) = listener.accept().await?;

            let server_task_ref = server.clone();
  
            tokio::task::spawn(async move {
                Builder::new()
                    .serve_connection(stream, service_fn(|hyper_req: hyper::Request<hyper::body::Incoming>| {
                        async {
                            let response = match prep_request(hyper_req).await {
                                Ok(request) => server_task_ref.handle(request).await,
                                Err(err) => server_task_ref.handle_err(err)
                            };

                            let hyper_resp = prep_response(response).await;

                            Ok::<_, hyper::Error>(hyper_resp)
                        }
                    }))
                    .await.unwrap() // TODO: Unwrap.
            });
        }
    }
}
