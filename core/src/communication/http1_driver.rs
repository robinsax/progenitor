use async_trait::async_trait;
use bytes::Bytes;
use tokio::{task, net::TcpListener, io::Error as TokioIoError};
use hyper::{service::service_fn, server::conn::http1::Builder, Error}; // TODO lose?
use http_body_util::{Full, BodyExt};

use super::common::{BindOptions, CommunicationError};
use super::handlers::{Route, Request, RootHandler, Response};
use super::driver::{CommunicationDriver};

impl From<TokioIoError> for CommunicationError {
    fn from(_: TokioIoError) -> Self {
        CommunicationError::TODO
    }
}

impl From<Error> for CommunicationError {
    fn from(_: Error) -> Self {
        CommunicationError::TODO
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
impl CommunicationDriver for Http1CommunicationDriver {
    async fn handle_connections(&mut self, handler: &Box<dyn RootHandler>) -> Result<(), CommunicationError> {
        let listener = TcpListener::bind(self.bind_options.address).await?;

        loop {
            let (stream, _) = listener.accept().await?;
  
            let handler_ref = handler.clone();

            task::spawn(async {
                Builder::new()
                    .serve_connection(stream, service_fn(move |hyper_request: hyper::Request<hyper::body::Incoming>| {
                        let future_handler_ref = handler_ref.clone(); // TODO deal with this, double move = brutal

                        async move {
                            Ok::<_, hyper::Error>(prepare_response(
                                match prepare_request(hyper_request).await {
                                    Ok(req) => {
                                        match future_handler_ref.handle_sync(&req).await {
                                            Ok(response) => response,
                                            Err(err) => err.into()
                                        }
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

