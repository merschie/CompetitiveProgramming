use std::cmp;

pub struct SegTree {
    pub value: i32,
    pub left: Option<Box<SegTree>>,
    pub tl: usize,
    pub tr: usize,
    pub right: Option<Box<SegTree>>,
}

impl SegTree
where
    i32: Copy,
{
    pub fn new(value: i32, tl: usize, tr: usize) -> Self {
        SegTree {
            value,
            left: None,
            right: None,
            tl,
            tr,
        }
    }

    pub fn create(&mut self, arr: Vec<i32>) -> Self {
        println!("Create Segmentation Tree with array: {:?}", arr);
        kleiner(arr, 0)
    }

    pub fn left(mut self, node: SegTree) -> Self {
        self.left = Some(Box::new(node));
        self
    }

    pub fn right(mut self, node: SegTree) -> Self {
        self.right = Some(Box::new(node));
        self
    }

    pub fn preorder(&self) {
        println!("V:{},l:{},r:{}", self.value, self.tl, self.tr);

        if !(self.left.is_none()) {
            self.left.as_ref().unwrap().preorder();
        }
        if !(self.right.is_none()) {
            self.right.as_ref().unwrap().preorder();
        }
    }
    pub fn update(mut self, i: usize, j: usize, t: i32) {
        println!("i:{} j:{}", i, j);
        println!("tl:{} tr:{}", self.tl, self.tr);
        if self.tl == self.tr {
            println!("Updating");
            self.value = cmp::min(t, self.value)
        }

        let mid = if let 0 = j - i % 2 {
            (j - i / 2) + i
        } else {
            (j - i / 2) + 1 + i
        };

        //left

        if mid < self.tr {
            self.left.unwrap().update(i, j, t);
        } else if mid > self.tl {
            self.right.unwrap().update(i, j, t);
        }

        //right
    }
}

pub fn kleiner(arr: Vec<i32>, offset: usize) -> SegTree {
    if arr.len() == 2 {
        let x = create_leave(arr[0], arr[1], offset);
        return x;
    }

    if arr.len() == 3 {
        let x = SegTree::new(arr[0] + arr[1] + arr[2], offset, offset + 2)
            .right(SegTree::new(arr[2], offset + 2, offset + 2))
            .left(create_leave(arr[0], arr[1], offset));
        return x;
    }

    let mid = if let 0 = arr.len() % 2 {
        arr.len() / 2
    } else {
        (arr.len() / 2) + 1
    };

    let start = Vec::from_iter(arr[..mid].iter().cloned());
    let end = Vec::from_iter(arr[mid..].iter().cloned());

    let left = kleiner(start, offset);
    let right = kleiner(end, offset + mid);

    let root = left.value + right.value;
    let lt = left.tl;
    let rt = right.tr;

    SegTree::new(root, lt, rt).right(right).left(left)
}

pub fn create_leave(left: i32, right: i32, offset: usize) -> SegTree {
    let x = SegTree::new(left + right, offset, offset + 1)
        .left(SegTree::new(left, offset, offset))
        .right(SegTree::new(right, offset + 1, offset + 1));
    return x;
}
