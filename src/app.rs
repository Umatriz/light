use std::rc::Rc;

use tokio::net::TcpListener;

use crate::{core::Core, prelude::Result};

#[derive(Debug)]
pub struct App {
    addr: String,
    clusters: Vec<Box<dyn Core>>,
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
