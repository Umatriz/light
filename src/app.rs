use crate::clusters::Cluster;

use self::builder::AppBuilder;

pub mod builder;

#[derive(Debug)]
pub struct App {
    addr: String,
    clusters: Vec<Cluster>,
}

impl App {}
