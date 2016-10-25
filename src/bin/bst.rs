
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

    bs = bs.insert(6);
    bs = bs.insert(3);
    bs = bs.insert(2);
    bs = bs.insert(1);
    bs = bs.insert(4);
    bs = bs.insert(5);
    bs = bs.insert(8);
    bs = bs.insert(7);

    // .-8
    // | `-7
    // 6
    // |   .-5
    // | .-4
    // `-3
    //   `-2
    //     `-1

    println!("{:?}", bs.contains(4));
    println!("{:?}", bs.contains(10));

    for int in bs.into_iter() {
        println!("{}", int);
    }
}
