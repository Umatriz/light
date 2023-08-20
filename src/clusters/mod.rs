use std::rc::Rc;

use tokio::net::TcpStream;

use self::service::Service;

pub mod service;

pub async fn check_cluster() {}

#[derive(Default, Debug)]
pub struct Cluster {
    root_path: String,
    services: Vec<Box<dyn Service>>,
    clusters: Vec<Cluster>,
}

impl Cluster {
    pub fn process(stream: TcpStream) {}
}
