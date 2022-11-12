use std::cmp;
#[derive(Clone)]
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
        //println!("Create Segmentation Tree with array: {:?}", arr);
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
    pub fn update(mut self, i: usize, j: usize, t: i32) -> Self {
        //println!("Update, i={}, j={} ,T={},", i, j, t);
        let mid = ((self.tr - self.tl) / 2) + self.tl;
        //println!("Schaue bei i:{} j:{} mit Mitte:{}", i, j, mid);

        //bei einem Element angekommen
        if self.tr == self.tl {
            //println!("nur noch ein Element");
            self.value = cmp::min(t, self.value);
            return self;
        }

        //muss getrennt werden
        if i <= mid && mid < j {
            //println!("muss trennen");
            let left = self.left.clone().unwrap();
            let right = self.right.clone().unwrap();
            return self
                .left(left.update(i, mid, t))
                .right(right.update(mid, j, t));
        }

        //passt ganz links rein

        if j <= mid {
            //println!("found left");
            self.value = cmp::max(t, self.right.as_ref().unwrap().value);
            let left = self.left.clone().unwrap();
            return self.left(left.update(i, j, t));
        }

        //passt ganz rechts rein
        if i > mid {
            //println!("found right");
            self.value = cmp::max(t, self.left.as_ref().unwrap().value);
            let right = self.right.clone().unwrap();
            return self.right(right.update(i, j, t));
        }

        //println!("Hier läuft was falsch!!!");

        self
    }

    pub fn max(&self, i: usize, j: usize) -> i32 {
        let mid = ((self.tr - self.tl) / 2) + self.tl;
        //println!("Schaue bei i:{} j:{} mit Mitte:{}", i, j, mid);
        //bei einem Element angekommen
        if self.tr == self.tl {
            //println!("nur noch ein Element");
            let x = self.value;
            return x;
        }

        //muss getrennt werden
        if i <= mid && mid < j {
            //println!("muss trennen");
            let left = self.left.as_ref().unwrap().max(i, mid);
            let right = self.right.as_ref().unwrap().max(mid + 1, j);

            if left < right {
                return right;
            }
            return left;
        }

        //passt ganz links rein

        if j <= mid {
            //println!("found left");
            return self.left.as_ref().unwrap().max(i, j);
        }

        //passt ganz rechts rein
        if i >= mid {
            //println!("found right");
            return self.right.as_ref().unwrap().max(i, j);
        }

        println!("Hier läuft was falsch!!!");
        return 0;
    }
}

pub fn kleiner(arr: Vec<i32>, offset: usize) -> SegTree {
    if arr.len() == 2 {
        let x = create_leave(arr[0], arr[1], offset);
        return x;
    }

    if arr.len() == 3 {
        let x = SegTree::new(
            cmp::max(cmp::max(arr[0], arr[1]), arr[2]),
            offset,
            offset + 2,
        )
        .right(SegTree::new(arr[2], offset + 2, offset + 2))
        .left(create_leave(arr[0], arr[1], offset));
        return x;
    }

    let mid = find_mid(0, arr.len());

    let start = Vec::from_iter(arr[..mid].iter().cloned());
    let end = Vec::from_iter(arr[mid..].iter().cloned());

    let left = kleiner(start, offset);
    let right = kleiner(end, offset + mid);

    let root = cmp::max(left.value, right.value);
    let lt = left.tl;
    let rt = right.tr;

    SegTree::new(root, lt, rt).right(right).left(left)
}

pub fn create_leave(left: i32, right: i32, offset: usize) -> SegTree {
    let x = SegTree::new(cmp::max(left, right), offset, offset + 1)
        .left(SegTree::new(left, offset, offset))
        .right(SegTree::new(right, offset + 1, offset + 1));
    return x;
}

pub fn find_mid(a: usize, b: usize) -> usize {
    let mid = if let 0 = (b - a) % 2 {
        ((b - a) / 2) + a
    } else {
        ((b - a) / 2) + 1 + a
    };
    return mid;
}
