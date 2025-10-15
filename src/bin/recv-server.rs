use std::path::Path;

use anyhow::{Context, Result};
use s2n_quic::{Server};


#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug"))
        .init();

    let mut server = Server::builder()
        .with_tls((Path::new("./certs/server-cert.pem"), Path::new("./certs/server-key.pem")))?
        .with_io("0.0.0.0:4433")?
        .start()
        .context("Failed to start server")?;

    log::debug!("Server started on {}", server.local_addr()?);

    while let Some(mut connection) = server.accept().await {
        log::debug!("Accepted connection from {}", connection.remote_addr()?);

        tokio::spawn(async move {
            while let Ok(Some(mut stream)) = connection.accept_bidirectional_stream().await {
                tokio::spawn(async move {
                    while let Ok(Some(data)) = stream.receive().await {
                        if let Err(e) = stream.send(data).await.context("Failed to send the data") {
                            log::error!("Failed to send the data: {:?}", e);
                            break;
                        }
                            
                    }
                });
            }
        });
    }
    Ok(())
}