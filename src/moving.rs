
#[derive(Debug)]
pub struct BST<T: Ord> {
    pub root: Option<Box<Node<T>>>
}

#[derive(Debug)]
pub struct Node<T: Ord> {
    item: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Ord> BST<T> {
    pub fn new() -> Self {
        BST { root: None }
    }

    pub fn insert(self, e: T) -> Self {
        BST {
            root: match self.root {
                None => Some(Box::new(Node::new(e))),
                Some(node) => Some(Box::new(node.insert(e))),
            }
        }
    }

    pub fn contains(&self, e: T) -> bool {
        self.root.as_ref().map(|node| node.contains(e)).unwrap_or(false)
    }
}

impl<T: Ord> Node<T> {
    fn new(e: T) -> Self {
        Node { item: e, left: None, right: None }
    }

    fn insert(self, e: T) -> Self {
        let Node { item, left, right } = self;
        if e <= item {
            Node {
                item: item,
                right: right,
                left: match left {
                    None       => Some(Box::new(Node::new(e))),
                    Some(node) => Some(Box::new(node.insert(e))),
                }
            }
        } else {
            Node {
                item: item,
                left: left,
                right: match right {
                    None       => Some(Box::new(Node::new(e))),
                    Some(node) => Some(Box::new(node.insert(e))),
                }
            }
        }
    }

    fn contains(&self, e: T) -> bool {
        self.item == e || if e < self.item {
            self.left.as_ref().map(|node| node.contains(e)).unwrap_or(false)
        } else {
            self.right.as_ref().map(|node| node.contains(e)).unwrap_or(false)
        }
    }
}
