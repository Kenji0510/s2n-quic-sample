use std::{net::SocketAddr, path::Path};

use anyhow::{Context, Result};
use bytes::Bytes;
use s2n_quic::{client::Connect, Client};
use s2n_quic_sample::{operate_file::load_pcd_xyz, types};


#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug"))
        .init();

    // Load the pcd file
    let pcd_path = "data/input/removed-ceiling-clipped-cloud_registered_0_xyz.pcd";
    let points = load_pcd_xyz(pcd_path)
        .context("Failed to load PCD file")?;

    log::debug!("Loaded {} points from PCD file", points.len());

    let packet = types::PointCloudPacket::new(points.len(), points);
    // let json_packet = packet.to_json_bytes()?;
    // log::debug!("Serialized PointCloudPacket to JSON, size: {} bytes", json_packet.len());

    let send_data = bincode::serialize(&packet).context("Failed to transform the data to binary")?;
    log::debug!("Serialized PointCloudPacket to binary, size: {} bytes", send_data.len());

    let address: SocketAddr = "0.0.0.0:0".parse()?;

    // set up an io provider with jumbo mtu and larger socket buffers
    let io = s2n_quic::provider::io::Default::builder()
        .with_max_mtu(1228)?
        .with_receive_address(address)?
        .build()?;

    let client = Client::builder()
        .with_tls(Path::new("./certs/ca-cert.pem"))?
        .with_io(io)?
        .start()
        .context("Failed to start client")?;

    let addr: SocketAddr = "192.168.0.36:4433".parse()
        .context("Failed to parse client ip")?;
    let connect = Connect::new(addr).with_server_name("ikaros");
    // let start = std::time::Instant::now();
    let mut connection = client.connect(connect).await
        .context("Failed to connect the server")?;

    connection.keep_alive(true)?;

    let start = std::time::Instant::now();

    log::debug!("Client started on {}", client.local_addr()?);

    // open a new stream and split the receiving and sending sides
    let stream = connection.open_bidirectional_stream().await?;
    let (mut receive_stream, mut send_stream) = stream.split();

    send_stream.send(Bytes::from(send_data)).await
        .context("Failed to send the json data")?;
    log::debug!("Sent the data to the server");

    send_stream.finish()?;
    log::debug!("Finished sending the data to the server");

    while let Ok(Some(data)) = receive_stream.receive().await {
        log::debug!("{:?}", data);
    }

    connection.close(0u32.into());  
    log::debug!("Closed the connection");

    let duration = start.elapsed();
    log::debug!("Elapsed time: {:?}", duration);

    Ok(())
}