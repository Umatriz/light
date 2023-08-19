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

#[derive(Clone, Default)]
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
    pub fn addr(&self, addr: impl Into<String>) -> AppBuilder<Yes> {
        AppBuilder {
            addr: addr.into(),
            clusters: self.clusters.clone(),
            __marker_addr: PhantomData,
        }
    }
}

impl<A> AppBuilder<A> {
    pub fn add_cluster(&self, cluster: Cluster) -> AppBuilder<A> {
        let mut clusters = self.clusters.clone();
        clusters.push(cluster);
        AppBuilder {
            addr: self.addr.clone(),
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn app_builder_test() {
        let app = AppBuilder::new()
            .addr("127.0.0.1:8080")
            .add_cluster(Cluster)
            .add_cluster(Cluster)
            .build();

        println!("{:#?}", app)
    }
}
