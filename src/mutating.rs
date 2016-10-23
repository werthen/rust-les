pub struct BST<T> {
    pub root: Option<Box<Node<T>>>,
}

#[derive(Debug, PartialEq)]
pub struct Node<T> {
    val: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>
}

impl<T: Ord + Clone> BST<T> {
    pub fn new() -> Self {
        BST { root: Option::None }
    }

    pub fn insert(&mut self, val: T) {
        match self.root {
            None => {
                self.root = Some(Box::new(Node {
                    val: val,
                    left: None,
                    right: None,
                }))
            }
            Some(ref mut node) => node.insert(val),
        }
    }

    pub fn contains(&mut self, x: T) -> bool {
        match self.root {
            None => false,
            Some(ref mut node) => node.contains(x),
        }
    }
}

impl<T: Ord> Node<T> {
    pub fn insert(&mut self, x: T) {
        if x == self.val {
            return
        }

        let target = if self.val < x { &mut self.left } else { &mut self.right };

        match target {
            &mut Some(ref mut node) => node.insert(x),
            &mut None => {
                *target = Some(Box::new(Node {
                    val: x,
                    left: None,
                    right: None,
                }))
            }
        }
    }

    pub fn contains(&mut self, x: T) -> bool {
        if self.val == x {
            return true
        }

        let target = if self.val < x { &mut self.left } else { &mut self.right };

        match target {
            &mut Some(ref mut subnode) => subnode.contains(x),
            &mut None => false
        }
    }
}
