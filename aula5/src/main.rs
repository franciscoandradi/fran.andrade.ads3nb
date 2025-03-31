#[derive(Debug)]

struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>
}

#[derive(Debug)]
struct BST {
    root: Option<Box<Node>>
}

impl BST {
    fn new() -> Self {
        BST {root: None}
    }

    fn insert(&mut self, value: i32) {
        
    }
}