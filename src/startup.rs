use crate::cmd::{Cli, Command};
use crate::discovery::{discovery_request_service, peer_discovery_service};
use crate::peer::shared_peer_type;
use crate::sample::rand_msg::{generate_random_text, get_random_name};
use clap::Parser;
use std::io::Write;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, UdpSocket};
use tokio::time;
pub async fn init_app() {
    let cmd = Cli::parse().command.unwrap();
    match &cmd {
        Command::Discovery { port } => {
            startup(port.clone()).await;
        }
    };
}

pub async fn startup(port: String) {
    let local_ip = local_ip_address::local_ip().unwrap().to_string();
    println!("Local ip:{}", local_ip);
    let peer_list: shared_peer_type = Arc::new(Mutex::new(Vec::new()));
    let discovery_port = port.parse().unwrap();
    let peer_name = get_random_name();
    let cloned_local_ip = local_ip.to_string();
    let discovery_socket = UdpSocket::bind(("0.0.0.0", discovery_port)).await.unwrap();
    let tcp_listener = TcpListener::bind("0.0.0.0:0").await.unwrap();
    let tcp_listener_port = tcp_listener.local_addr().unwrap().port();
    let discovery_request_handle = tokio::spawn(discovery_request_service(
        discovery_socket,
        tcp_listener_port,
    ));

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
            time::sleep(Duration::from_secs(20)).await;
        }
    });
    let cloned_peer_list = Arc::clone(&peer_list);
    let tcp_write_handler = tokio::spawn(async move {
        loop {
            let msg = peer_name.clone() + ": " + &generate_random_text();
            println!("{} is Sending to all peers: {}", cloned_local_ip, msg);
            let peer_list = cloned_peer_list.lock().unwrap().clone();
            peer_list.iter().for_each(|addr| {
                let mut sock = TcpStream::connect(addr).unwrap();
                sock.write_all(msg.as_bytes()).unwrap();
            });

            time::sleep(Duration::from_secs(20)).await;
        }
    });
    let tcp_read_handler = tokio::spawn(async move {
        loop {
            let (mut s, addr) = tcp_listener.accept().await.unwrap();

            let mut buf = [0; 1024];
            let len = s.read(&mut buf).await.unwrap();
            let addr = addr.ip().to_string() + ":" + &addr.port().to_string();
            println!(
                "Received from {} --> {}",
                addr,
                String::from_utf8(Vec::from(&buf[..len])).unwrap()
            );
        }
    });

    tokio::try_join!(
        discovery_request_handle,
        peer_discovery_handle,
        peer_print_handle,
        tcp_write_handler,
        tcp_read_handler,
    )
    .unwrap();
}
