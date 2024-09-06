mod config;
mod splice;

use std::os::fd::AsRawFd;
use std::ptr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::task;

use crate::splice::{splice, SPLICE_F_MOVE};

async fn handle_client(incoming: TcpStream, target_addr: &str) -> std::io::Result<()> {
    let outgoing = TcpStream::connect(target_addr).await?;
    let fd_in = incoming.as_raw_fd();
    let fd_out = outgoing.as_raw_fd();

    task::spawn_blocking(move || {
        loop {
            let result = unsafe {
                splice(fd_in, ptr::null_mut(), fd_out, ptr::null_mut(), 65536, SPLICE_F_MOVE)
            };

            if result <= 0 {
                break;
            }
        }
    }).await?;

    Ok(())
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
