use async_trait::async_trait;
use bytes::Bytes;
use tokio::sync::RwLock;
use tokio::{task, net::TcpListener, io::Error as TokioIoError};
use hyper::{service::service_fn, server::conn::http1::Builder, Error}; // TODO lose?
use http_body_util::{Full, BodyExt};

use crate::archetype::LiteralValue;

use super::super::SerialJson;
use super::RootHandler;
use super::common::{BindOptions, CommunicationError};
use super::handlers::{SyncHandler, Route, Request, Response};
use super::driver::{CommunicationDriverFactory, CommunicationDriver};

impl From<TokioIoError> for CommunicationError {
    fn from(_: TokioIoError) -> Self {
        CommunicationError::TODO
    }
}

pub struct Http1CommunicationDriverFactory {
    
}

impl From<Error> for CommunicationError {
    fn from(_: Error) -> Self {
        CommunicationError::TODO
    }
}

#[async_trait]
impl CommunicationDriverFactory for Http1CommunicationDriverFactory {
    async fn create_server(&self, options: &BindOptions) -> Result<Box<dyn CommunicationDriver>, CommunicationError> {
        Ok(Box::new(Http1CommunicationDriver{ bind_options: options.clone() }))
    }
}

pub struct Http1CommunicationDriver {
    bind_options: BindOptions,
}

// TODO so bad and json lock
async fn convert_req(hyper_req: hyper::Request<hyper::body::Incoming>) -> Result<Request, CommunicationError> {
    let path = hyper_req.uri().clone().to_string();

    let raw = hyper_req.collect().await?.to_bytes();
    let ser: SerialJson = Bytes::from(raw).try_into()?;

    let payload: LiteralValue = ser.try_into()?;

    Ok(Request { route: Route::new(path), payload })
}

async fn convert_resp(resp: Response) -> hyper::Response<Full<Bytes>> {
    hyper::Response::new(match resp {
        Response::Success(data) => {
            let ser: SerialJson = data.try_into().unwrap();

            Full::new(ser.try_into().unwrap())
        },
        Response::Error(err, detail) => {
            Full::new(Bytes::from("error"))
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
                        let future_handler_ref = handler_ref.clone(); // TODO deal with this

                        async move {
                            let request: Request = convert_req(hyper_request).await.unwrap();
                            let response = future_handler_ref.handle_sync(&request).await;
                        
                            Ok::<_, hyper::Error>(convert_resp(response).await)
                        }
                    }))
                    .await.unwrap() // TODO no
            });
        }
    }
}

