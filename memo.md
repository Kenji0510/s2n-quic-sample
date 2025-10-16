## Sent the pcd file to the server on inside
```bash
kenji@kenji-RTX4080:~/workspace/rust/s2n-quic-sample$ cargo run --release --bin send-client
    Finished `release` profile [optimized] target(s) in 0.04s
     Running `target/release/send-client`
[2025-10-16T06:53:30Z DEBUG send_client] Loaded 8486 points from PCD file
[2025-10-16T06:53:30Z DEBUG send_client] Serialized PointCloudPacket to JSON, size: 390223 bytes
[2025-10-16T06:53:30Z DEBUG send_client] Client started on 0.0.0.0:57950
[2025-10-16T06:53:30Z DEBUG send_client] Sent the data to the server
[2025-10-16T06:53:30Z DEBUG send_client] Finished sending the data to the server
[2025-10-16T06:53:30Z DEBUG send_client] Closed the connection
[2025-10-16T06:53:30Z DEBUG send_client] Elapsed time: 1.134754ms
```

## Sent the pcd from agx to nano
```bash
root@ubuntu:~/s2n-quic-sample# cargo run --release --bin send-client
    Finished `release` profile [optimized] target(s) in 0.13s
     Running `target/release/send-client`
[2025-10-16T09:22:01Z DEBUG send_client] Loaded 16063 points from PCD file
[2025-10-16T09:22:01Z DEBUG send_client] Serialized PointCloudPacket to JSON, size: 730151 bytes
[2025-10-16T09:22:01Z DEBUG send_client] Client started on 0.0.0.0:2198
[2025-10-16T09:22:01Z DEBUG send_client] Sent the data to the server
[2025-10-16T09:22:01Z DEBUG send_client] Finished sending the data to the server
[2025-10-16T09:22:01Z DEBUG send_client] Closed the connection
[2025-10-16T09:22:01Z DEBUG send_client] Elapsed time: 15.531951ms
```

## Resolved server can't send return packet to client
```bash
kenji@ubuntu:~/workspace/rust/s2n-quic-sample$ sudo tcpdump -i eno1 -vvv -s0 -n udp port 4433
tcpdump: listening on eno1, link-type EN10MB (Ethernet), snapshot length 262144 bytes
18:12:13.958536 IP (tos 0x2,ECT(0), ttl 64, id 0, offset 0, flags [DF], proto UDP (17), length 1228)
    192.168.100.153.2198 > 192.168.100.152.4433: [udp sum ok] UDP, length 1200
18:12:13.964027 IP (tos 0x2,ECT(0), ttl 64, id 0, offset 0, flags [DF], proto UDP (17), length 1549)
    192.168.100.152.4433 > 192.168.100.153.2198: [bad udp cksum 0x508d -> 0x22ef!] UDP, length 1521
18:12:13.966069 IP (tos 0x2,ECT(0), ttl 64, id 0, offset 0, flags [DF], proto UDP (17), length 1228)
    192.168.100.153.2198 > 192.168.100.152.4433: [udp sum ok] UDP, length 1200
18:12:13.987591 IP (tos 0x2,ECT(0), ttl 64, id 0, offset 0, flags [DF], proto UDP (17), length 105)
    192.168.100.153.2198 > 192.168.100.152.4433: [udp sum ok] UDP, length 77
18:12:13.989069 IP (tos 0x2,ECT(0), ttl 64, id 0, offset 0, flags [DF], proto UDP (17), length 2428)
    192.168.100.152.4433 > 192.168.100.153.2198: [bad udp cksum 0x53fc -> 0x3f5c!] UDP, length 2400
18:12:14.009321 IP (tos 0x2,ECT(0), ttl 64, id 0, offset 0, flags [DF], proto UDP (17), length 2428)
    192.168.100.152.4433 > 192.168.100.153.2198: [bad udp cksum 0x53fc -> 0x051c!] UDP, length 2400
18:12:14.030120 IP (tos 0x2,ECT(0), ttl 64, id 0, offset 0, flags [DF], proto UDP (17), length 105)
    192.168.100.153.2198 > 192.168.100.152.4433: [udp sum ok] UDP, length 77
18:12:14.030144 IP (tos 0x2,ECT(0), ttl 64, id 0, offset 0, flags [DF], proto UDP (17), length 105)
    192.168.100.153.2198 > 192.168.100.152.4433: [udp sum ok] UDP, length 77
18:12:14.031345 IP (tos 0x2,ECT(0), ttl 64, id 0, offset 0, flags [DF], proto UDP (17), length 105)
    192.168.100.152.4433 > 192.168.100.153.2198: [bad udp cksum 0x4ae9 -> 0x8520!] UDP, length 77
18:12:14.049464 IP (tos 0x2,ECT(0), ttl 64, id 0, offset 0, flags [DF], proto UDP (17), length 2428)
    192.168.100.152.4433 > 192.168.100.153.2198: [bad udp cksum 0x53fc -> 0x0c3f!] UDP, length 2400
18:12:14.129864 IP (tos 0x0, ttl 64, id 0, offset 0, flags [DF], proto UDP (17), length 2428)
    192.168.100.152.4433 > 192.168.100.153.2198: [bad udp cksum 0x53fc -> 0x16cb!] UDP, length 2400
18:12:14.288604 IP (tos 0x0, ttl 64, id 0, offset 0, flags [DF], proto UDP (17), length 2428)
    192.168.100.152.4433 > 192.168.100.153.2198: [bad udp cksum 0x53fc -> 0xe006!] UDP, length 2400
18:12:14.605636 IP (tos 0x0, ttl 64, id 0, offset 0, flags [DF], proto UDP (17), length 2428)
    192.168.100.152.4433 > 192.168.100.153.2198: [bad udp cksum 0x53fc -> 0xc344!] UDP, length 2400
18:12:15.237788 IP (tos 0x0, ttl 64, id 0, offset 0, flags [DF], proto UDP (17), length 2428)
    192.168.100.152.4433 > 192.168.100.153.2198: [bad udp cksum 0x53fc -> 0x6224!] UDP, length 2400
18:12:16.501906 IP (tos 0x0, ttl 64, id 0, offset 0, flags [DF], proto UDP (17), length 2428)
    192.168.100.152.4433 > 192.168.100.153.2198: [bad udp cksum 0x53fc -> 0x7a6a!] UDP, length 2400
18:12:19.028305 IP (tos 0x0, ttl 64, id 0, offset 0, flags [DF], proto UDP (17), length 2428)
    192.168.100.152.4433 > 192.168.100.153.2198: [bad udp cksum 0x53fc -> 0xb6e7!] UDP, length 2400
^C
16 packets captured
17 packets received by filter
0 packets dropped by kernel
kenji@ubuntu:~/workspace/rust/s2n-quic-sample$ sudo ethtool -k eno1 | egrep -i 'gso|gro|tso|ufo|udp|segm'
tcp-segmentation-offload: off
        tx-tcp-segmentation: off
        tx-tcp-ecn-segmentation: off [fixed]
        tx-tcp-mangleid-segmentation: off
        tx-tcp6-segmentation: off [fixed]
generic-segmentation-offload: off
tx-gso-robust: off [fixed]
tx-fcoe-segmentation: off [fixed]
tx-gre-segmentation: off [fixed]
tx-gre-csum-segmentation: off [fixed]
tx-ipxip4-segmentation: off [fixed]
tx-ipxip6-segmentation: off [fixed]
tx-udp_tnl-segmentation: off [fixed]
tx-udp_tnl-csum-segmentation: off [fixed]
tx-gso-partial: off [fixed]
tx-tunnel-remcsum-segmentation: off [fixed]
tx-sctp-segmentation: off [fixed]
tx-esp-segmentation: off [fixed]
tx-udp-segmentation: on
tx-gso-list: off [fixed]
rx-udp_tunnel-port-offload: off [fixed]
rx-gro-hw: off [fixed]
rx-gro-list: off
rx-udp-gro-forwarding: off
kenji@ubuntu:~/workspace/rust/s2n-quic-sample$ # 代表的なオフロード無効化（送信側が重要）
sudo ethtool -K eno1 tx-udp-segmentation off   # (= udpgso)
sudo ethtool -K eno1 tx-udptnl-segmentation off
sudo ethtool -K eno1 tx-gso-partial off
sudo ethtool -K eno1 gso off
sudo ethtool -K eno1 tso off
sudo ethtool -K eno1 gro off
# 反映確認
sudo ethtool -k eno1 | egrep -i 'gso|gro|tso|udp|segm'
netlink error: bit name not found (offset 56)
netlink error: Operation not supported
tcp-segmentation-offload: off
        tx-tcp-segmentation: off
        tx-tcp-ecn-segmentation: off [fixed]
        tx-tcp-mangleid-segmentation: off
        tx-tcp6-segmentation: off [fixed]
generic-segmentation-offload: off
tx-gso-robust: off [fixed]
tx-fcoe-segmentation: off [fixed]
tx-gre-segmentation: off [fixed]
tx-gre-csum-segmentation: off [fixed]
tx-ipxip4-segmentation: off [fixed]
tx-ipxip6-segmentation: off [fixed]
tx-udp_tnl-segmentation: off [fixed]
tx-udp_tnl-csum-segmentation: off [fixed]
tx-gso-partial: off [fixed]
tx-tunnel-remcsum-segmentation: off [fixed]
tx-sctp-segmentation: off [fixed]
tx-esp-segmentation: off [fixed]
tx-udp-segmentation: off
tx-gso-list: off [fixed]
rx-udp_tunnel-port-offload: off [fixed]
rx-gro-hw: off [fixed]
rx-gro-list: off
rx-udp-gro-forwarding: off
```
https://chatgpt.com/c/68efc47d-2b4c-8323-bac1-a52f94b8edaa