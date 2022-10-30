fn start_within_http1_driver() {
    let addr: SocketAddr = "127.0.0.1:8000".parse().or_else(
        |_| Err(NetError::TODO)
    )?;

    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);
    loop {
        let (stream, _) = listener.accept().await?;

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(stream, service_fn(hello))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}
