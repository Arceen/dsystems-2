use clap::{Parser, Subcommand};
use std::net::SocketAddr;
use tokio::net::UdpSocket;
use tokio::time::{Duration, timeout};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    commands: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    Client {
        #[arg(short, long)]
        port: String,
    },
}

#[tokio::main]
async fn main() {
    let cmd = Cli::parse().commands.unwrap();
    match &cmd {
        Command::Client { port } => {
            let local_port: u16 = port.parse().unwrap();

            // Start the discovery responder in the background
            let responder_socket = UdpSocket::bind(("0.0.0.0", 8888)).await.unwrap();
            tokio::spawn(discovery_responder(responder_socket, local_port));

            // Wait a bit for other peers to start up
            tokio::time::sleep(Duration::from_secs(2)).await;

            // Discover peers
            let peers = discover_peers(local_port).await.unwrap();
            println!("Peer on port {} discovered: {:#?}", port, peers);

            // Start peer-to-peer communication
            run_p2p_client(local_port, peers).await;
        }
    }
}

// Responds to discovery requests
async fn discovery_responder(socket: UdpSocket, local_port: u16) {
    let mut buf = [0; 1024];
    loop {
        if let Ok((len, addr)) = socket.recv_from(&mut buf).await {
            if &buf[..len] == b"P2P_DISCOVER" {
                println!(
                    "Port {} received discovery request from {}",
                    local_port, addr
                );
                let _ = socket.send_to(b"P2P_RESPONSE", addr).await;
            }
        }
    }
}

async fn discover_peers(local_port: u16) -> Result<Vec<SocketAddr>, Box<dyn std::error::Error>> {
    // Use a different port for discovery to avoid conflicts
    let discovery_socket = UdpSocket::bind(("0.0.0.0", 0)).await?;
    discovery_socket.set_broadcast(true)?;

    // Send discovery message to the Docker network broadcast address
    // For subnet 191.0.0.0/24, broadcast is 191.0.0.255
    discovery_socket
        .send_to(b"P2P_DISCOVER", "191.0.0.255:8888")
        .await?;

    let mut peers = vec![];
    let mut buf = [0; 1024];

    // Use timeout to avoid infinite waiting
    let discovery_timeout = Duration::from_secs(5);
    let start_time = std::time::Instant::now();

    while start_time.elapsed() < discovery_timeout {
        if let Ok(result) = timeout(
            Duration::from_millis(100),
            discovery_socket.recv_from(&mut buf),
        )
        .await
        {
            if let Ok((len, addr)) = result {
                if &buf[..len] == b"P2P_RESPONSE" {
                    // Don't add ourselves
                    if addr.port() != local_port {
                        peers.push(addr);
                        println!("Found peer: {}", addr);
                    }
                }
            }
        }
    }

    Ok(peers)
}

async fn run_p2p_client(local_port: u16, peers: Vec<SocketAddr>) {
    if peers.is_empty() {
        println!("No peers found, running in standalone mode");
        return;
    }

    // Create a new socket for communication
    let comm_socket = UdpSocket::bind(("0.0.0.0", 0)).await.unwrap();
    let mut msg_count: usize = 0;

    loop {
        // Send message to all discovered peers
        for peer in &peers {
            let message = format!("Hello from port {}, message #{}", local_port, msg_count);
            let _ = comm_socket.send_to(message.as_bytes(), peer).await;
        }

        msg_count += 1;

        // Try to receive messages (non-blocking)
        let mut buf = vec![0; 1024];
        if let Ok(result) =
            timeout(Duration::from_millis(100), comm_socket.recv_from(&mut buf)).await
        {
            if let Ok((len, addr)) = result {
                let received_msg = String::from_utf8_lossy(&buf[..len]);
                println!(
                    "Port {} received: '{}' from {}",
                    local_port, received_msg, addr
                );
            }
        }

        tokio::time::sleep(Duration::from_secs(2)).await;
    }
}
