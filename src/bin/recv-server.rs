use std::{net::SocketAddr, path::{Path, PathBuf}, sync::{Arc}};

use anyhow::{Context, Result};
use s2n_quic::{Server};
use s2n_quic_sample::{operate_file::save_pcd_xyz, types::{PointCloudPacket}};
use tokio::{sync::{mpsc, Mutex}, task};

#[derive(Debug)]
struct SaveJob {
    bytes: Vec<u8>,
    path: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug"))
        .init();

    let save_workers = start_save_workers(2);

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

        let save_worker_tx = save_workers.clone();
        tokio::spawn(async move {
            while let Ok(Some(mut stream)) = connection.accept_bidirectional_stream().await {
                let save_worker_tx = save_worker_tx.clone();
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

                    // let packet = match bincode::deserialize::<PointCloudPacket>(&buf) {
                    //     Ok(packet) => {
                    //         log::debug!("Deserialized PointCloudPacket with {} points", packet.points.len());
                    //         packet
                    //     }
                    //     Err(e) => {
                    //         log::error!("Failed to deserialize PointCloudPacket: {:?}", e);
                    //         return;
                    //     }
                    // };

                    if let Err(e) = save_worker_tx.send(
                        SaveJob { bytes: buf, path: PathBuf::from("data/output/received_points.pcd") }
                    ).await {
                        log::error!("Failed to send save job to worker: {:?}", e);
                    }

                    // match save_pcd_xyz(&packet.points, "data/output/received_points.pcd") {
                    //     Ok(_) => log::debug!("Successfully saved PCD file"),
                    //     Err(e) => log::error!("Failed to save received pcd: {:?}", e),
                    // }
                });
                
            }

            log::debug!("Connection from {} closed", &remote_addr);
        });
        
    }
    Ok(())
}

fn start_save_workers(n: usize) -> mpsc::Sender<SaveJob> {
    let (tx, rx) = mpsc::channel::<SaveJob>(8);
    let rx = Arc::new(Mutex::new(rx));

    for _ in 0..n {
        let rx = rx.clone();
        tokio::spawn(async move {
            loop {
                let job = {
                    let mut receiver = rx.lock().await;
                    receiver.recv().await
                };

                match job {
                    Some(job) => {
                        let res = task::spawn_blocking(move || -> Result<()> {
                            let packet = bincode::deserialize::<PointCloudPacket>(&job.bytes)
                                .context("Failed to deserialize the received data")?;

                            save_pcd_xyz(&packet.points, &job.path.as_path().to_str().unwrap())
                                .context("Failed to save PCD file")?;

                            log::debug!("Successfully saved PCD file to {:?}", job.path);
                            Ok(())
                        })
                        .await;

                        match res {
                            Ok(Ok(())) => {
                                log::debug!("Worker completed save job");
                            }
                            Ok(Err(e)) => {
                                log::error!("Worker save error: {:?}", e);
                            }
                            Err(e) => {
                                log::error!("Worker spawn_blocking error: {:?}", e);
                            }
                        }
                    }
                    None => {
                        log::debug!("Worker shutting down - channel closed");
                        break;
                    }
                }
            }
        });
    }

    tx
}