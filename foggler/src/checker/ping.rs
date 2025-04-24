use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};
use std::time::Instant;

pub async fn ping(host: &str, port: u16, timeout_secs: u64) -> Result<u64, u64> {
    let address: String = format!("{}:{}", host, port);
    let start: Instant = Instant::now();
    let timeout_duration: Duration = Duration::from_secs(timeout_secs);

    match timeout(timeout_duration, TcpStream::connect(&address)).await {
        Ok(Ok(_)) => Ok(start.elapsed().as_millis() as u64),
        Ok(Err(e)) => Err(-1),
        Err(_) => Err(-1),
    }
}
