use std::net::SocketAddr;

use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
};
use tracing::{info, warn};

const BUFFER_SIZE: usize = 4096;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    info!("starting dredis server");
    // build a listener
    let addr = "0.0.0.0:6379";
    let listener = TcpListener::bind(addr).await?;
    info!("listening on {}", addr);
    loop {
        let (stream, raddr) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = process_redis_conn(stream, raddr).await {
                warn!("error processing connection {}: {:?}", raddr, e);
            }
        });
    }
    #[allow(unreachable_code)]
    Ok(())
}

async fn process_redis_conn(mut stream: TcpStream, raddr: SocketAddr) -> anyhow::Result<()> {
    info!("accepted connection from {}", raddr);
    loop {
        stream.readable().await?;

        let mut buf = Vec::with_capacity(BUFFER_SIZE);
        match stream.try_read_buf(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                info!("read {} bytes", n);
                let line = String::from_utf8_lossy(&buf);
                info!("{:?}", line);
                stream.write_all(b"+OK\r\n").await?;
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    Ok(())
}
