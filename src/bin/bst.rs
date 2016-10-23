
extern crate rustles;

use rustles::moving;
use rustles::mutating;

fn main() {
    mutating();
    moving();
}

fn mutating() {
    let mut bs = mutating::BST::<i32>::new();
    for x in vec![3, 4, 5, 6] {
        bs.insert(x);
        println!("{:?}", bs.root);
    }

    println!("{:?}", bs.contains(4));
    println!("{:?}", bs.contains(10));
}

fn moving() {
    let mut bs = moving::BST::<i32>::new();
    for x in vec![3, 4, 5, 6] {
        bs = bs.insert(x);
        println!("{:?}", bs.root);
    }

    println!("{:?}", bs.contains(4));
    println!("{:?}", bs.contains(10));
}
