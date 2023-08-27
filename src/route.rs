use std::collections::BTreeMap;
use std::sync::{Arc, RwLock, Weak};

// #[derive(Debug, Default)]
// struct Tree<T> {
//     tree: Vec<Arc<Node<T>>>,
// }

// #[derive(Debug)]
// struct Node<T> {
//     payload: T,
//     parent: Arc<RwLock<Weak<Node<T>>>>,
//     children: Arc<Vec<RwLock<Arc<Node<T>>>>>,
// }

#[derive(Debug, Default)]
struct Tree<T> {
    tree: Vec<Arc<RwLock<Node<T>>>>,
}

#[derive(Debug)]
struct Node<T> {
    payload: T,
    parent: Weak<RwLock<Node<T>>>,
    children: Vec<Arc<RwLock<Node<T>>>>,
}

#[test]
fn node_test() {
    let node = Arc::new(RwLock::new(Node::<i32> {
        payload: 1,
        parent: Weak::new(),
        children: vec![],
    }));

    let tree = Tree {
        tree: vec![Arc::new(RwLock::new(Node::<i32> {
            payload: 0,
            parent: Weak::new(),
            children: vec![node.clone()],
        }))],
    };

    let binding = node;
    let mut r = binding.write().unwrap();

    r.parent = Arc::downgrade(&tree.tree[0].clone());

    std::thread::spawn(move || println!("{:#?}", tree));
}
