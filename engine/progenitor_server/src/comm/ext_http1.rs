use std::net::SocketAddr;
use std::sync::Arc;

use bytes::Bytes;
use tokio::net::TcpListener;
use tokio::task::{LocalSet, spawn_local};
use tokio::runtime::Runtime;
use http_body_util::{Full, BodyExt};

// TODO: Likely a temporary dependency.
use hyper::{service::service_fn, server::conn::http1::Builder};

use progenitor::{InitError, SerialValue, Registry};

use super::errors::CommError;
use super::server::Server;
use super::driver::CommDriver;
use super::io::{Request, Response, Route};

impl From<tokio::io::Error> for CommError {
    fn from(err: tokio::io::Error) -> Self {
        Self::Interface(format!("<tokio::io {:?}>", err))
    }
}

impl From<hyper::Error> for CommError {
    fn from(err: hyper::Error) -> Self {
        Self::Interface(format!("<hyper:: {:?}>", err))
    }
}

// TODO so bad
async fn prep_request(hyper_req: hyper::Request<hyper::body::Incoming>) -> Result<Request, CommError> {
    let path = hyper_req.uri().clone().to_string();
    let raw = hyper_req.collect().await?.to_bytes();

    Ok(Request::new(Route::new(path), SerialValue::Buffer(raw)))
}

async fn prep_response(resp: Response) -> hyper::Response<Full<Bytes>> {
    hyper::Response::new(
        match resp.payload().try_into_bytes() {
            Ok(bytes) => Full::new(bytes),
            Err(_) => {
                todo!("TODO: Response error handling");
            }
        }
    )
}

#[derive(Clone)] // TODO: No.
pub struct Http1Comm {
    socket_addr: SocketAddr
}

impl CommDriver for Http1Comm {
    fn new(registry: Arc<Registry>) -> Result<Self, InitError> {
        let socket_addr = match registry.get_config("http1_sock")?.parse::<SocketAddr>() {
            Ok(addr) => addr,
            Err(_) => return Err(InitError::Config("invalid socket addr".into()))
        };

        Ok(Self {
            socket_addr
        })
    }

    // TODO: Non-thread local (big time investment).
    fn handle_connections(&self, server: Server<Http1Comm>) -> Result<(), CommError> {
        let runtime = match Runtime::new() {
            Ok(runtime) => runtime,
            Err(err) => return Err(CommError::Interface(err.to_string()))
        };
        let local = LocalSet::new();

        local.block_on(&runtime, async move {
            let listener = TcpListener::bind(self.socket_addr).await?;

            loop {
                let (stream, _) = listener.accept().await?;

                let server_task_ref = server.clone();
    
                spawn_local(async move {
                    Builder::new()
                        .serve_connection(stream, service_fn(|hyper_req: hyper::Request<hyper::body::Incoming>| {
                            async {
                                let response = match prep_request(hyper_req).await {
                                    Ok(request) => server_task_ref.handle(request).await,
                                    Err(err) => server_task_ref.err_response(err)
                                };

                                let hyper_resp = prep_response(response).await;

                                Ok::<_, hyper::Error>(hyper_resp)
                            }
                        }))
                        .await.unwrap() // TODO: Unwrap.
                });
            }
        })
    }
}
