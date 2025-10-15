use std::{net::SocketAddr, path::Path};

use anyhow::{Context, Result};
use s2n_quic::{client::Connect, Client};


#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug"))
        .init();

    let client = Client::builder()
        .with_tls(Path::new("./certs/ca-cert.pem"))?
        .with_io("0.0.0.0:0")?
        .start()
        .context("Failed to start client")?;

    let addr: SocketAddr = "127.0.0.1:4433".parse()
        .context("Failed to parse client ip")?;
    let connect = Connect::new(addr).with_server_name("localhost");
    let mut connection = client.connect(connect).await
        .context("Failed to connect the server")?;

    connection.keep_alive(true)?;

    log::debug!("Client started on {}", client.local_addr()?);

    // open a new stream and split the receiving and sending sides
    let stream = connection.open_bidirectional_stream().await?;
    let (mut receive_stream, mut send_stream) = stream.split();

    // spawn a task that copies responses from the server to stdout
    tokio::spawn(async move {
        let mut stdout = tokio::io::stdout();
        let _ = tokio::io::copy(&mut receive_stream, &mut stdout).await;
    });

    // copy data from stdin and send it to the server
    let mut stdin = tokio::io::stdin();
    tokio::io::copy(&mut stdin, &mut send_stream).await?;

    Ok(())
}