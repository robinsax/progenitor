use std::net::SocketAddr;
use std::sync::Arc;

use async_trait::async_trait;
use bytes::Bytes;
use tokio::{task, net::TcpListener, io::Error as TokioIoError};
use hyper::{service::service_fn, server::conn::http1::Builder, Error}; // TODO lose?
use http_body_util::{Full, BodyExt};

use super::common::{BindOptions, CommunicationError};
use super::handlers::{Route, Request, SyncHandler, Response};
use super::driver::{CommunicationDriver};

impl From<TokioIoError> for CommunicationError {
    fn from(err: TokioIoError) -> Self {
        CommunicationError::TODO(format!("comm from tokio io {:?}", err))
    }
}

impl From<Error> for CommunicationError {
    fn from(err: Error) -> Self {
        CommunicationError::TODO(format!("comm from hyper {:?}", err))
    }
}

pub struct Http1CommunicationDriver {
    bind_options: BindOptions,
}

// TODO so bad
async fn prepare_request(hyper_req: hyper::Request<hyper::body::Incoming>) -> Result<Request, CommunicationError> {
    let path = hyper_req.uri().clone().to_string();
    let raw = hyper_req.collect().await?.to_bytes();

    Ok(Request { route: Route::new(path), payload: raw.into() })
}

async fn prepare_response(resp: Response) -> hyper::Response<Full<Bytes>> {
    hyper::Response::new(match resp {
        Response::Success(serial) => {
            Full::new(serial.into())
        },
        Response::Error(err) => {
            Full::new(Bytes::from(format!("TODO {:?}", err)))
        }
    })
}

#[async_trait]
impl<H: SyncHandler + 'static> CommunicationDriver<H> for Http1CommunicationDriver {
    fn new() -> Self {
        Self {
            bind_options: BindOptions { address: "0.0.0.0:8000".parse().unwrap() }
        }
    }

    async fn handle_connections(self) -> Result<(), CommunicationError> {
        let listener = TcpListener::bind(self.bind_options.address).await?;

        loop {
            let (stream, _) = listener.accept().await?;
  
            task::spawn(async {
                Builder::new()
                    .serve_connection(stream, service_fn(|hyper_request: hyper::Request<hyper::body::Incoming>| {
                        async {
                            Ok::<_, hyper::Error>(prepare_response(
                                match prepare_request(hyper_request).await {
                                    Ok(req) => match H::new().handle_sync(req).await {
                                        Ok(resp) => resp,
                                        Err(err) => err.into()
                                    },
                                    Err(err) => err.into()
                                }
                            ).await)
                        }
                    }))
                    .await.unwrap() // TODO no
            });
        }
    }
}
