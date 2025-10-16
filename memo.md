```bash
kenji@kenji-RTX4080:~/workspace/rust/s2n-quic-sample$ cargo run --release --bin send-client
    Finished `release` profile [optimized] target(s) in 0.04s
     Running `target/release/send-client`
[2025-10-15T17:10:28Z DEBUG send_client] Loaded 8486 points from PCD file
[2025-10-15T17:10:28Z DEBUG send_client] Client started on 0.0.0.0:42019
[2025-10-15T17:10:28Z DEBUG send_client] Sent the data to the server
[2025-10-15T17:10:28Z DEBUG send_client] Finished sending the data to the server
[2025-10-15T17:10:28Z DEBUG send_client] Closed the connection
[2025-10-15T17:10:28Z DEBUG send_client] Elapsed time: 33.944µs
```

```bash
kenji@kenji-RTX4080:~/workspace/rust/s2n-quic-sample$ cargo run --release --bin send-client
   Compiling s2n-quic-sample v0.1.0 (/home/kenji/workspace/rust/s2n-quic-sample)
    Finished `release` profile [optimized] target(s) in 5.54s
     Running `target/release/send-client`
[2025-10-15T17:11:00Z DEBUG send_client] Loaded 8486 points from PCD file
[2025-10-15T17:11:00Z DEBUG send_client] Client started on 0.0.0.0:44058
[2025-10-15T17:11:00Z DEBUG send_client] Sent the data to the server
[2025-10-15T17:11:00Z DEBUG send_client] Finished sending the data to the server
[2025-10-15T17:11:00Z DEBUG send_client] Closed the connection
[2025-10-15T17:11:00Z DEBUG send_client] Elapsed time: 1.090183ms
```