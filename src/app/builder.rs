use std::marker::PhantomData;

use crate::clusters::Cluster;

mod markers {
    #[derive(Clone, Default)]
    pub struct Yes;

    #[derive(Clone, Default)]
    pub struct No;
}

use markers::*;

use super::App;

#[derive(Default)]
pub struct AppBuilder<Addr = No> {
    addr: String,
    clusters: Vec<Cluster>,
    __marker_addr: PhantomData<Addr>,
}

impl AppBuilder {
    pub fn new() -> Self {
        AppBuilder::default()
    }
}

impl AppBuilder<No> {
    pub fn addr(self, addr: impl Into<String>) -> AppBuilder<Yes> {
        AppBuilder {
            addr: addr.into(),
            clusters: self.clusters,
            __marker_addr: PhantomData,
        }
    }
}

impl<A> AppBuilder<A> {
    pub fn add_cluster(self, cluster: Cluster) -> AppBuilder<A> {
        let mut clusters = self.clusters;
        clusters.push(cluster);
        AppBuilder {
            addr: self.addr,
            clusters,
            __marker_addr: PhantomData,
        }
    }
}

impl AppBuilder<Yes> {
    pub fn build(self) -> App {
        App {
            addr: self.addr,
            clusters: self.clusters,
        }
    }
}
