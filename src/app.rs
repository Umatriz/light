use std::rc::Rc;

use tokio::net::TcpListener;

use crate::clusters::Cluster;

use crate::prelude::Result;

pub mod builder;

#[derive(Debug)]
pub struct App {
    addr: String,
    clusters: Vec<Cluster>,
}

impl App {
    pub async fn start(self) -> Result<()> {
        let listener = TcpListener::bind(self.addr.clone()).await?;

        let clusters = Rc::new(self.clusters);

        loop {
            let (stream, _) = listener.accept().await?;
        }
    }
}
