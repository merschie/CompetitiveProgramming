use problem2::SegTree;
use std::io::{self, BufRead};
fn main() {
    let stdin = io::stdin();

    let input = stdin.lock().lines().next().unwrap().unwrap();
    let mut x = input.split(" ");
    let n: usize = x.next().unwrap().parse().unwrap();
    let mut m: usize = x.next().unwrap().parse().unwrap();

    let arr = stdin.lock().lines().next().unwrap().unwrap();
    let arr: Vec<i32> = arr
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();

    let mut add_tree = SegTree::new(1, 0, n - 1);
    println!("hi");
    let mut add_tree = add_tree.create(arr);
    //add_tree.preorder();

    println!("{m}");
    let mut o = Vec::new();
    let mut operation_array = vec![0; m];
    let mut operation_tree = SegTree::new(1, 0, m);
    let mut operation_tree = operation_tree.create(operation_array);
    //operation_tree.preorder();

    for line in stdin.lock().lines() {
        let mut numbers = line.as_ref().unwrap().split(" ");
        if m > 0 {
            let l: usize = numbers.next().unwrap().parse().unwrap();
            let r: usize = numbers.next().unwrap().parse().unwrap();
            let n: i32 = numbers.next().unwrap().parse().unwrap();
            m = m - 1;
            o.push((l - 1, r - 1, n));
        } else {
            let i: usize = numbers.next().unwrap().parse().unwrap();
            let j: usize = numbers.next().unwrap().parse().unwrap();
            operation_tree = operation_tree.update(i - 1, j - 1, 1);
            operation_tree.preorder();
            //make tree for this
            // for x in i..j + 1 {
            //     println!("execute O({x}):{:?}", o[x - 1]);
            //     operation_array[x - 1] = operation_array[x - 1] + 1
            // }
            println!("----------");
        }
    }
    operation_array = operation_tree.give_array(0);
    println!("*********************");
    add_tree.preorder();
    for x in 0..operation_array.len() {
        let (l, r, n) = o[x];
        println!(
            "execute O({x}) {:?}-times. We will execute it once and add {} insted",
            operation_array[x],
            operation_array[x] * n
        );
        add_tree = add_tree.update(l, r, operation_array[x] * n);
    }
    for x in add_tree.give_array(0) {
        print!("{x} ");
    }
}
