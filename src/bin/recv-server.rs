use std::{net::SocketAddr, path::Path};

use anyhow::{Context, Result};
use s2n_quic::{Server};
use s2n_quic_sample::{operate_file::save_pcd_xyz, types::{PointCloudPacket}};


#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug"))
        .init();

    let address: SocketAddr = "0.0.0.0:4433".parse()?;

    // set up an io provider with jumbo mtu and larger socket buffers
    let io = s2n_quic::provider::io::Default::builder()
        .with_receive_address(address)?
        .with_max_mtu(1228)?
        .build()?;

    let mut server = Server::builder()
        .with_tls((Path::new("./certs/server-cert.pem"), Path::new("./certs/server-key.pem")))?
        .with_io(io)?
        .start()
        .context("Failed to start server")?;

    log::debug!("Server started on {}", server.local_addr()?);

    while let Some(mut connection) = server.accept().await {
        let mut total_data_size = 0;
        let remote_addr = connection.remote_addr()?;
        log::debug!("Accepted connection from {}", &remote_addr);

        tokio::spawn(async move {
            while let Ok(Some(mut stream)) = connection.accept_bidirectional_stream().await {
                tokio::spawn(async move {
                    let mut buf = Vec::new();

                    while let Ok(Some(data)) = stream.receive().await {
                        // log::debug!("Received {} bytes", data.len());
                        // if let Err(e) = stream.send(Bytes::from("Server received!")).await.context("Failed to send the data") {
                        //     log::error!("Failed to send the data: {:?}", e);
                        //     break;
                        // }
                        total_data_size += data.len();
                        buf.extend_from_slice(&data);
                    }
                    
                    let _ = stream.finish().context("Failed to finish the stream");
                    log::debug!("Total received data size: {} bytes", total_data_size);

                    let packet = match bincode::deserialize::<PointCloudPacket>(&buf) {
                        Ok(packet) => {
                            log::debug!("Deserialized PointCloudPacket with {} points", packet.points.len());
                            packet
                        }
                        Err(e) => {
                            log::error!("Failed to deserialize PointCloudPacket: {:?}", e);
                            return;
                        }
                    };

                    match save_pcd_xyz(&packet.points, "data/output/received_points.pcd") {
                        Ok(_) => log::debug!("Successfully saved PCD file"),
                        Err(e) => log::error!("Failed to save received pcd: {:?}", e),
                    }
                });
                
            }

            log::debug!("Connection from {} closed", &remote_addr);
        });
        
    }
    Ok(())
}