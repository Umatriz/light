use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
};

mod app;
mod core;
mod endpoint;
mod http;

pub mod prelude;

pub async fn req() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (stream, _) = listener.accept().await?;
        process_socket(stream).await;
    }
}

async fn process_socket(mut stream: TcpStream) {
    let mut buf = vec![0; 1024];
    let m = stream.read(&mut buf).await.unwrap();

    println!("Req: {}", String::from_utf8_lossy(&buf[..m]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn req_test() {
        let r = tokio::runtime::Builder::new_multi_thread()
            .enable_io()
            .build()
            .unwrap();

        r.block_on(req()).unwrap();
    }
}
