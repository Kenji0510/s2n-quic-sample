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
    let pcd_path = "data/input/removed-ceiling-output-003.pcd";
    let points = load_pcd_xyz(pcd_path)
        .context("Failed to load PCD file")?;

    log::debug!("Loaded {} points from PCD file", points.len());

    let packet = types::PointCloudPacket::new(points.len(), points);
    // let json_packet = packet.to_json_bytes()?;
    // log::debug!("Serialized PointCloudPacket to JSON, size: {} bytes", json_packet.len());

    let send_data = bincode::serialize(&packet).context("Failed to transform the data to binary")?;
    log::debug!("Serialized PointCloudPacket to binary, size: {} bytes", send_data.len());

    let server_addr = "192.168.0.36:4433";
    let server_name = "ikaros";
    
    send_via_quic(send_data, server_addr, server_name).await?;

    Ok(())
}

async fn send_via_quic(
    data: Vec<u8>,
    server_addr: &str,
    server_name: &str,
) -> Result<()> {
    // let start = std::time::Instant::now();

    // クライアントのアドレス
    let local_address: SocketAddr = "0.0.0.0:0".parse()?;

    // IO プロバイダーの設定
    let io = s2n_quic::provider::io::Default::builder()
        .with_max_mtu(1228)?
        .with_receive_address(local_address)?
        .build()?;

    // クライアントの作成
    let client = Client::builder()
        .with_tls(Path::new("./certs/ca-cert.pem"))?
        .with_io(io)?
        .start()
        .context("Failed to start client")?;

    log::debug!("Client started on {}", client.local_addr()?);

    // サーバーへの接続
    let addr: SocketAddr = server_addr.parse()
        .context("Failed to parse server address")?;
    let connect = Connect::new(addr).with_server_name(server_name);
    
    let mut connection = client.connect(connect).await
        .context("Failed to connect to the server")?;

    let start = std::time::Instant::now();

    connection.keep_alive(true)?;

    // ストリームを開く
    let stream = connection.open_bidirectional_stream().await?;
    let (mut receive_stream, mut send_stream) = stream.split();

    // データ送信
    send_stream.send(Bytes::from(data)).await
        .context("Failed to send the data")?;
    log::debug!("Sent the data to the server");

    // 送信完了
    send_stream.finish()
        .context("Failed to finish sending")?;
    log::debug!("Finished sending the data to the server");

    // サーバーからのレスポンスを受信
    while let Ok(Some(response)) = receive_stream.receive().await {
        log::debug!("Received response: {:?}", response);
    }

    // コネクションを閉じる
    connection.close(0u32.into());
    log::debug!("Closed the connection");

    let duration = start.elapsed();
    log::debug!("Elapsed time: {:?}", duration);

    Ok(())
}