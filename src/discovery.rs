use crate::peer::shared_peer_type;
use std::time;
use std::time::Duration;
use tokio::net::UdpSocket;
use tokio::time::timeout;

pub async fn peer_discovery_service(
    client_socket: UdpSocket,
    discovery_port: u16,
    local_ip: String,
    peer_list: shared_peer_type,
) {
    println!("discovery port: {}", client_socket.local_addr().unwrap());
    // allowing the socket to send boradcast requests to the gateway
    client_socket.set_broadcast(true).unwrap();

    tokio::spawn(async move {
        loop {
            client_socket
                .send_to(b"P2P_REQ", format!("255.255.255.255:{}", discovery_port))
                .await
                .unwrap();

            let discovery_start_instance = time::Instant::now();
            let discovery_timeout = time::Duration::from_secs(5);
            println!("Waiting for discovery for {:#?} seconds", discovery_timeout);

            let mut peers = vec![];
            while discovery_start_instance.elapsed() < discovery_timeout {
                let mut buf = [0; 1024];
                if let Ok(Ok((len, addr))) =
                    timeout(Duration::from_millis(5), client_socket.recv_from(&mut buf)).await
                {
                    let res_str = String::from_utf8(buf[..len].to_owned()).unwrap();
                    let mut split_str = res_str.split("\n");
                    let (req_header, tcp_port) = (split_str.next(), split_str.next());

                    if b"P2P_RES" == req_header.unwrap().as_bytes() {
                        let tcp_addr = addr.ip().to_string()
                            + ":"
                            + tcp_port.unwrap().split(":").last().unwrap();
                        peers.push(tcp_addr);
                    }
                }
            }
            let peers: Vec<String> = peers
                .into_iter()
                .filter(|addr| !addr.starts_with(&local_ip))
                .collect();
            *peer_list.lock().unwrap() = peers;
            // broadcase request to peers
            tokio::time::sleep(tokio::time::Duration::from_secs(15)).await;
        }
    })
    .await
    .unwrap();
}

pub async fn discovery_request_service(discovery_socket: UdpSocket, tcp_port: u16) {
    println!("Creating the discovery service");
    tokio::spawn(async move {
        let mut buf = [0; 1025];

        while let Ok((len, addr)) = discovery_socket.recv_from(&mut buf).await {
            if b"P2P_REQ" == &buf[..len] {
                let _ = discovery_socket
                    .send_to(
                        format!("P2P_RES\nTCP_PORT:{}", { tcp_port }).as_bytes(),
                        addr,
                    )
                    .await;
            }
        }
    })
    .await
    .unwrap();
}
