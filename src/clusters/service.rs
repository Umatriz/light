use std::fmt::Debug;

use http::Response;
use tokio::net::TcpStream;

use bytes::Bytes;

#[async_trait::async_trait]
pub trait Service
where
    Self: Debug,
{
    async fn service_fn(&self, stream: TcpStream) -> Response<Bytes>;

    fn path(&self) -> String;
}
