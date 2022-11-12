use problem1::SegTree;
use std::io::{self, BufRead};
fn main() {
    let stdin = io::stdin();

    let nm = stdin.lock().lines().next().unwrap().unwrap();
    let mut x = nm.split(" ");
    let n: usize = x.next().unwrap().parse().unwrap();

    let arr = stdin.lock().lines().next().unwrap().unwrap();
    let arr: Vec<i32> = arr
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();

    let mut root = SegTree::new(1, 0, n);
    let mut root = root.create(arr);

    for line in stdin.lock().lines() {
        let mut numbers = line.as_ref().unwrap().split(" ");
        let case = numbers.next();
        if case == Some("0") {
            let i: usize = numbers.next().unwrap().parse().unwrap();
            let j: usize = numbers.next().unwrap().parse().unwrap();
            let t: i32 = numbers.next().unwrap().parse().unwrap();
            root = root.update(i - 1, j - 1, t);
        } else {
            let i: usize = numbers.next().unwrap().parse().unwrap();
            let j: usize = numbers.next().unwrap().parse().unwrap();
            println!("{}", root.max(i - 1, j - 1));
        }
    }
}
