use problem1::SegTree;
use std::io::{self, BufRead};
fn main() {
    let stdin = io::stdin();

    //read n and m

    let nm = stdin.lock().lines().next().unwrap().unwrap();
    let mut x = nm.split(" ");
    let n: usize = x.next().unwrap().parse().unwrap();
    let m: i32 = x.next().unwrap().parse().unwrap();
    //read data for tree

    let arr = stdin.lock().lines().next().unwrap().unwrap();
    let arr: Vec<i32> = arr
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();

    //create Tree
    //let mut tree: SegTree<n> = SegTree::new();
    //let mut root = SegTree::new(1).left(SegTree::new(3)).right(SegTree::new(4));
    let mut root = SegTree::new(1, 0, n);
    let mut root = root.create(arr);
    //root.preorder();
    root.update(3, 3, 30);
    //root.preorder();
    //read Commands

    // for line in stdin.lock().lines() {
    //     let mut numbers = line.as_ref().unwrap().split(" ");
    //     let case = numbers.next();
    //     if case == Some("0") {
    //         let i: i32 = numbers.next().unwrap().parse().unwrap();
    //         let j: i32 = numbers.next().unwrap().parse().unwrap();
    //         let T: i32 = numbers.next().unwrap().parse().unwrap();
    //         println!("Update, i={}, j={} ,T={},", i, j, T);
    //     } else {
    //         let i: i32 = numbers.next().unwrap().parse().unwrap();
    //         let j: i32 = numbers.next().unwrap().parse().unwrap();
    //         println!("Max, i={}, j={}", i, j);
    //     }
    //}
}
