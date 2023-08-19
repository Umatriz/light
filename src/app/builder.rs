use std::marker::PhantomData;

use crate::clusters::Cluster;

mod markers {
    pub struct Yes;
    pub struct No;
}

#[derive(Clone, Default)]
pub struct AppBuilder {
    addr: String,
    clusters: Vec<Cluster>,
}
