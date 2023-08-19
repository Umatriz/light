use crate::clusters::Cluster;

use self::builder::AppBuilder;

pub mod builder;

pub struct App {
    addr: String,
    clusters: Vec<Cluster>,
}

impl App {}
