
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

    let mut verify = Vec::with_capacity(30);
    for x in 1 .. 9  { verify.push(x) }
    for x in 0 .. 20 { verify.push((13 * x) % 100); bs = bs.insert((13 * x) % 100); }
    verify.sort();

    let mut i = 0;
    for int in bs.into_iter() {
        println!("{}  {:?}", int, verify.get(i));
        i += 1;
    }
}
