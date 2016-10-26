
use std::mem;

#[derive(Debug)]
pub struct BST<T> {
    pub root: Edge<T>,
}

type Edge<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct Node<T> {
    pub item: T,
    pub left: Edge<T>,
    pub right: Edge<T>,
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
    pub fn new(e: T) -> Self {
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
            let Node { item, left: _, right} = unboxed;
            if let Some(right) = right {
                self.remainder.push(right);
            }
            item
        })
    }
}

// More complex, but perhaps useful for the mutating BST? That one should have an Iter (and an
// IterMut), not a IntoIter.

//
// #[derive(PartialEq, Eq, Debug)]
// pub enum Direction { Left, Right }
// 
// #[derive(Debug)]
// pub struct InOrderIterator<T> {
//     path:    Vec<(T, Direction, Edge<T>)>,
//     subtree: Edge<T>,
// }
// 
// impl<T> InOrderIterator<T> {
//     // TODO none-public
//     pub fn new(tree: BST<T>) -> Self {
//         InOrderIterator {
//             path:    Vec::new(),
//             subtree: tree.root,
//         }
//     }
// 
//     pub fn go_left(&mut self) {
//         if let Some(boxnode) = mem::replace(&mut self.subtree, None) {
//             let node = *boxnode;
//             let Node { item: i, left: l, right: r } = node;
//             self.path.push((i, Direction::Left, r));
//             self.subtree = l;
//         }
//     }
// 
//     pub fn go_right(&mut self) {
//         if let Some(boxnode) = mem::replace(&mut self.subtree, None) {
//             let node = *boxnode;
//             let Node { item: i, left: l, right: r } = node;
//             self.path.push((i, Direction::Right, l));
//             self.subtree = r;
//         }
//     }
// 
//     pub fn go_leftmost(&mut self) {
//         while self.subtree.is_some() {
//             self.go_left();
//         }
//     }
// 
//     pub fn go_rightmost(&mut self) {
//         while self.subtree.is_some() {
//             self.go_right();
//         }
//     }
// 
//     pub fn go_up(&mut self) -> Option<Direction> {
//         self.path.pop().map(|(item, direction, sidetree)| {
//             let subtree = mem::replace(&mut self.subtree, None);
//             let (lefttree, righttree) = match direction {
//                 Direction::Left  => (subtree, sidetree),
//                 Direction::Right => (sidetree, subtree),
//             };
//             self.subtree = Some(Box::new(Node {
//                 item: item,
//                 left: lefttree,
//                 right: righttree,
//             }));
//             direction
//         })
//     }
// }
// 
// impl<T> Iterator for InOrderIterator<T> {
//     type Item = T;
// 
//     fn next(&mut self) -> Option<T> {
//         match self.subtree {
//             None => {
//                 while Some(Direction::Left) == self.go_up() {}
//                 self.subtree.map(|node| node.item)
//             }, // go up left *, go up right, yield
//             Some(ref node) => None, // go down right, go down left *, go up right, yield
//         }
//     }
// }

#[macro_export]
macro_rules! tree {
    (
        ($i:expr)
    ) => (
        $crate::moving::Node {
            item: $i,
            left:  None,
            right: None,
        }
    )
    ;
    (
            [ $l:tt ]
        ($i:expr)
            [ $r:tt ]
    ) => (
        $crate::moving::Node {
            item: $i,
            left:  Some(Box::new(tree!($l))),
            right: Some(Box::new(tree!($r))),
        }
    );
}
