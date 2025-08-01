use clap::{Parser, Subcommand};
use tokio::net::UdpSocket;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    commands: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    Client {
        #[arg(short, long)]
        send_port: String,
        #[arg(short, long)]
        recv_port: String,
    },
}
#[tokio::main]
async fn main() {
    let cmd = Cli::parse().commands.unwrap();
    match &cmd {
        Command::Client {
            send_port,
            recv_port,
        } => {
            run_client(send_port, recv_port).await;
        }
    }
    println!("{}", "forrest gump");
}

async fn run_client(send_port: &String, recv_port: &String) {
    let socket = UdpSocket::bind(format!("0.0.0.0:{}", recv_port))
        .await
        .unwrap();
    println!("client running on port: {recv_port}");
    let send_port = send_port.clone();
    tokio::spawn(async move {
        let mut msg_count: usize = 0;
        loop {
            let _ = socket
                .send_to(
                    format!("Client sending: {}", msg_count).as_bytes(),
                    &format!("0.0.0.0:{}", send_port),
                )
                .await
                .unwrap();
            msg_count += 1;
            let mut buf = vec![0; 33];
            let len = socket.recv(&mut buf).await.unwrap();
            println!("{}", String::from_utf8(buf[..len].to_owned()).unwrap());
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    })
    .await
    .unwrap();
}
