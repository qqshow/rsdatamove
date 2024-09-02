use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use prost::Message;
use transport::DataPacket;
use tracing::{info, error};
use tracing_subscriber;
use metrics::{increment_counter, register_counter};
use metrics_exporter_prometheus::PrometheusBuilder;

mod transport {
    include!(concat!(env!("OUT_DIR"), "/transport.rs"));
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing subscriber for logging
    tracing_subscriber::fmt::init();

    // Initialize Prometheus exporter for metrics
    let builder = PrometheusBuilder::new();
    let handle = builder.install_recorder().unwrap();

    // Register a counter for received packets
    register_counter!("received_packets");

    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    info!("Server listening on port 8080");

    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buf = vec![0; 1024];
            loop {
                let n = match socket.read(&mut buf).await {
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        error!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                let packet = DataPacket::decode(&buf[..n]).unwrap();
                info!("Received: {:?}", packet);

                // Increment the received packets counter
                increment_counter!("received_packets");

                if let Err(e) = socket.write_all(&buf[..n]).await {
                    error!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}