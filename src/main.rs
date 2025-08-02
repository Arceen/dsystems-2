/*
   Client sends a broadcast request to all the discoverable peers in the local network
   need a timeout to manage connection requests so that we don't do this all the time
*/
use clap::{Parser, Subcommand};
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, UdpSocket};
use tokio::time;
use tokio::time::{Duration, interval, timeout};

type shared_peer_type = Arc<Mutex<Vec<String>>>;
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    Discovery {
        #[arg(short, long)]
        port: String,
    },
}
#[tokio::main]
async fn main() {
    let local_ip = local_ip_address::local_ip().unwrap().to_string();
    println!("Local ip:{}", local_ip);
    let cmd = Cli::parse().command.unwrap();
    let peer_list: shared_peer_type = Arc::new(Mutex::new(Vec::new()));
    match &cmd {
        Command::Discovery { port } => {
            let discovery_port = port.parse().unwrap();
            let discovery_socket = UdpSocket::bind(("0.0.0.0", discovery_port)).await.unwrap();

            let discovery_request_handle =
                tokio::spawn(discovery_request_service(discovery_socket));

            let _tcp_listener = TcpListener::bind("0.0.0.0:0").await.unwrap();
            let discovery_client_socket = UdpSocket::bind(("0.0.0.0", 0)).await.unwrap();
            let cloned_peer_list = Arc::clone(&peer_list);
            let peer_discovery_handle = tokio::spawn(peer_discovery_service(
                discovery_client_socket,
                discovery_port,
                local_ip,
                cloned_peer_list,
            ));

            let cloned_peer_list = Arc::clone(&peer_list);
            let peer_print_handle = tokio::spawn(async move {
                loop {
                    println!("updated peer_list: {:#?}", cloned_peer_list.lock().unwrap());
                    time::sleep(time::Duration::from_secs(30)).await;
                }
            });

            tokio::try_join!(
                discovery_request_handle,
                peer_discovery_handle,
                peer_print_handle
            )
            .unwrap();
        }
    };
}

async fn peer_discovery_service(
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
                    if b"P2P_RES" == &buf[..len] {
                        peers.push(addr);
                    }
                }
            }
            let peers: Vec<String> = peers
                .iter()
                .map(|addr| addr.ip().to_string())
                .filter(|addr| addr != &local_ip)
                .collect();
            *peer_list.lock().unwrap() = peers;
            // broadcase request to peers
            tokio::time::sleep(tokio::time::Duration::from_secs(20)).await;
        }
    })
    .await
    .unwrap();
}

async fn discovery_request_service(discovery_socket: UdpSocket) {
    println!("Creating the discovery service");
    tokio::spawn(async move {
        let mut buf = [0; 1025];

        while let Ok((len, addr)) = discovery_socket.recv_from(&mut buf).await {
            if b"P2P_REQ" == &buf[..len] {
                let _ = discovery_socket.send_to(b"P2P_RES", addr).await;
            }
        }
    })
    .await
    .unwrap();
}
