
use std::mem;

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

pub struct InOrderIterator<T> {
    remainder: Vec<Box<Node<T>>>,
}

impl<T> IntoIterator for BST<T> {
    type Item = T;
    type IntoIter = InOrderIterator<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        InOrderIterator {
            remainder: self.root.into_iter().collect(),
        }
    }
}

impl<T> Iterator for InOrderIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.remainder.pop().map(|mut node| {
            while let Some(leftbox) = mem::replace(&mut node.left, None) {
                self.remainder.push(node);
                node = leftbox;
            }
            let unboxed = *node;
            let Node { val, left: _, right} = unboxed;
            if let Some(right) = right {
                self.remainder.push(right);
            }
            val
        })
    }
}

