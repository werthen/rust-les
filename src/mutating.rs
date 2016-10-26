pub struct BST<T> {
    pub root: Edge<T>
}

type Edge<T> = Option<Box<Node<T>>>;

#[derive(Debug, PartialEq)]
pub struct Node<T> {
    val: T,
    left: Edge<T>,
    right: Edge<T>
}

impl<T: Ord + Clone> BST<T> {
    pub fn new() -> Self {
        BST { root: None }
    }

    pub fn insert(&mut self, val: T) {
        match self.root {
            None => self.root = Some(Box::new(Node::new(val))),
            Some(ref mut node) => node.insert(val),
        }
    }

    pub fn contains(&self, x: T) -> bool {
        match self.root {
            None => false,
            Some(ref node) => node.contains(x),
        }
    }
}

impl<T: Ord> Node<T> {
    pub fn new(val: T) -> Self {
        Node { val: val, left: None, right: None }
    }

    pub fn insert(&mut self, x: T) {
        if x == self.val {
            return
        }

        let target = if self.val < x { &mut self.left } else { &mut self.right };

        match target {
            &mut Some(ref mut node) => node.insert(x),
            &mut None => *target = Some(Box::new(Node::new(x)))
        }
    }

    pub fn contains(&self, x: T) -> bool {
        if self.val == x {
            return true
        }

        let target = if self.val < x { &self.left } else { &self.right };

        match *target {
            Some(ref subnode) => subnode.contains(x),
            None => false
        }
    }
}
