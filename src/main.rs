mod config;

use std::sync::Arc;
use tokio::io::copy;
use tokio::net::{TcpListener, TcpStream};

async fn handle_client(mut incoming: TcpStream, target_addr: &str) -> std::io::Result<()> {
    let mut outgoing = TcpStream::connect(target_addr).await?;
    let (mut ri, mut wi) = incoming.split();
    let (mut ro, mut wo) = outgoing.split();

    let client_to_server = copy(&mut ri, &mut wo);
    let server_to_client = copy(&mut ro, &mut wi);

    tokio::select!(
        Err(err) = client_to_server => {
            Err(err)
        }
        Err(err) = server_to_client => {
            Err(err)
        }
    )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let config = config::AppConfig::load()?;
    let mut runners = Vec::new();
    for forward in config.forward {
        let listen_addr = Arc::new(format!("{}:{}", forward.listen_host, forward.listen_port));
        let target_addr = Arc::new(format!("{}:{}", forward.target_host, forward.target_port));

        let listener = TcpListener::bind(listen_addr.clone().as_str()).await?;
        tracing::info!("Listening on: {}, forwarding to: {}", &listen_addr, &target_addr);

        let handle = tokio::spawn(async move {
            loop {
                match listener.accept().await {
                    Ok((incoming, _)) => {
                        tracing::debug!("Accepted connection from: {:?}", incoming.peer_addr());

                        tokio::spawn({
                            let listen_addr = listen_addr.clone();
                            let target_addr = target_addr.clone();
                            async move {
                                if let Err(err) = handle_client(incoming, target_addr.as_str()).await {
                                    tracing::error!("[{} -> {}] Failed to handle connection: {:?}", listen_addr, target_addr, err);
                                }
                            }
                        });
                    }
                    Err(err) => {
                        tracing::error!("Failed to accept connection: {:?}", err);
                        continue;
                    }
                }
            }
        });

        runners.push(handle);
    }

    tracing::info!("Forwarding server initialized");

    futures::future::join_all(runners).await;

    Ok(())
}
