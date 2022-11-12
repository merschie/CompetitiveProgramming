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
        create_tree(arr, 0)
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
        //let mid = find_mid(self.tl, self.tr);
        let mid = ((self.tr - self.tl) / 2) + self.tl;

        println!("Look at i:{} j:{} with mid:{}", i, j, mid);
        println!("Borders: left:{} right:{}", self.tl, self.tr);

        //bei einem passendem Bereich angekommen
        if self.tl == i && self.tr == j {
            println!("only one element");
            self.value = self.value + t;
            return self;
        }

        //muss getrennt werden
        if i <= mid && mid < j {
            println!("divide!");
            let left = self.left.clone().unwrap();
            let right = self.right.clone().unwrap();
            return self
                .left(left.update(i, mid, t))
                .right(right.update(mid + 1, j, t));
        }

        if i == j {
            println!("go to single element");
            if self.tr <= j {
                let right = self.right.clone().unwrap();
                return self.right(right.update(i, j, t));
            } else if self.tl > i {
                let left = self.left.clone().unwrap();
                return self.left(left.update(i, j, t));
            }
            println!("Dont go here!!!");
        }

        //passt ganz links rein

        if j <= mid {
            println!("found left");
            //self.value = cmp::max(t, self.right.as_ref().unwrap().value);
            let left = self.left.clone().unwrap();
            return self.left(left.update(i, j, t));
        }

        //passt ganz rechts rein
        if i > mid {
            println!("found right");
            // self.value = cmp::max(t, self.left.as_ref().unwrap().value);
            let right = self.right.clone().unwrap();
            return self.right(right.update(i, j, t));
        }

        println!("Something is wrong!!!");

        self
    }

    pub fn give_array(&self, father: i32) -> Vec<i32> {
        println!("V:{},l:{},r:{}", self.value, self.tl, self.tr);
        let mut werte = Vec::new();
        if self.tl == self.tr {
            werte.push(self.value + father);
            return werte;
        }

        if !(self.left.is_none()) {
            werte.append(&mut self.left.as_ref().unwrap().give_array(self.value + father));
        }
        if !(self.right.is_none()) {
            werte.append(&mut self.right.as_ref().unwrap().give_array(self.value + father));
        }

        return werte;
    }
}

pub fn create_tree(arr: Vec<i32>, offset: usize) -> SegTree {
    if arr.len() == 1 {
        println!("gfliuhrebsolifguhsdrlig");
        return SegTree::new(arr[0], 0, 0);
    }

    if arr.len() == 2 {
        let x = create_leave(arr[0], arr[1], offset);
        return x;
    }

    if arr.len() == 3 {
        let x = SegTree::new(0, offset, offset + 2)
            .right(SegTree::new(arr[2], offset + 2, offset + 2))
            .left(create_leave(arr[0], arr[1], offset));
        return x;
    }

    let mid = find_mid(0, arr.len());

    let start = Vec::from_iter(arr[..mid].iter().cloned());
    let end = Vec::from_iter(arr[mid..].iter().cloned());

    let left = create_tree(start, offset);
    let right = create_tree(end, offset + mid);

    let lt = left.tl;
    let rt = right.tr;

    SegTree::new(0, lt, rt).right(right).left(left)
}

pub fn create_leave(left: i32, right: i32, offset: usize) -> SegTree {
    let x = SegTree::new(0, offset, offset + 1)
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
