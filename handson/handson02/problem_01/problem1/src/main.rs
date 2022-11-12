use problem1::SegTree;
use std::io::{self, BufRead};
fn main() {
    let stdin = io::stdin();

    //read n and m

    let nm = stdin.lock().lines().next().unwrap().unwrap();
    let mut x = nm.split(" ");
    let n: usize = x.next().unwrap().parse().unwrap();
    //let m: i32 = x.next().unwrap().parse().unwrap();
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
    //println!("Maximum: {}", root.max(2, 9));
    //println!("Maximum: {}", root.max(2, 9));
    //root.preorder();
    //read Commands
    let mut m = 0;
    let mut u = 0;
    for line in stdin.lock().lines() {
        let mut numbers = line.as_ref().unwrap().split(" ");

        let case = numbers.next();
        if case == Some("0") {
            let i: usize = numbers.next().unwrap().parse().unwrap();
            let j: usize = numbers.next().unwrap().parse().unwrap();
            let t: i32 = numbers.next().unwrap().parse().unwrap();
            root = root.update(i - 1, j - 1, t);
            u = u + 1;
            // root.preorder();
        } else {
            let i: usize = numbers.next().unwrap().parse().unwrap();
            let j: usize = numbers.next().unwrap().parse().unwrap();
            //println!("m:{}", m);
            println!("{}", root.max(i - 1, j - 1));
            m = m + 1;
        }
    }
}
