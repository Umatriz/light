use std::{
    fmt::Debug,
    sync::{Arc, RwLock, Weak},
};

use crate::prelude::Wrapper;

struct Node<T: Clone> {
    inner: Arc<RwLock<NodeData<T>>>,
}

#[derive(Debug)]
struct NodeData<T: Clone> {
    payload: T,
    parent: Weak<RwLock<NodeData<T>>>,
    children: Vec<Arc<RwLock<NodeData<T>>>>,
}

impl<T: Clone> Node<T> {
    pub fn new(payload: T) -> Self {
        Self {
            inner: Arc::new(RwLock::new(NodeData {
                payload,
                parent: Weak::new(),
                children: vec![],
            })),
        }
    }
}

impl<T: Clone + Debug> Node<T> {
    pub fn add_children(&mut self, node_data: Node<T>) {
        let binding: Arc<RwLock<NodeData<T>>> = node_data.inner.clone();
        let mut r_node_data = binding.write().unwrap();
        r_node_data.parent = Arc::downgrade(&self.inner);

        let mut children = self.inner.read().unwrap().children.clone();
        children.push(node_data.inner);

        let binding = &self.inner.clone();
        let mut self_locked = binding.write().unwrap();

        self_locked.children = children;
    }
}

#[test]
fn node_data_test() {
    let mut node_data = Node::new("payload");
    let c_node_data = Node::new("c");
    node_data.add_children(c_node_data);

    println!("{:#?}", node_data.inner.clone());
}
