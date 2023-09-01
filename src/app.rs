use std::rc::Rc;

use tokio::net::TcpListener;

use crate::{core::Core, prelude::Result};

#[derive(Debug)]
pub struct App {
    addr: String,
    cores: Vec,
}

pub struct AppBuilder {
    addr: String,
    cores: Vec<Box<Core>>,
}

impl App {
    pub async fn start(self) -> Result<()> {
        let listener = TcpListener::bind(self.addr.clone()).await?;

        let clusters = Rc::new(self.cores);

        loop {
            let (stream, _) = listener.accept().await?;
        }
    }
}
