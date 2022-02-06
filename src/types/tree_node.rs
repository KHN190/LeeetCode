use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Default)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

pub enum TraverseOrder {
    Inorder,
    Preorder,
    Postorder,
    LevelOrder,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn from_vec(v: &Vec<i32>) -> Self {
        assert!(v.len() > 0);
        let mut root = TreeNode::new(v[0]);
        root.inorder_insert(v, 0);
        root
    }

    pub fn set_lhs(&mut self, val: i32) -> &mut Option<Rc<RefCell<Self>>> {
        self.left = Some(Rc::new(RefCell::new(TreeNode::new(val))));
        &mut self.left
    }

    pub fn set_rhs(&mut self, val: i32) -> &mut Option<Rc<RefCell<Self>>> {
        self.right = Some(Rc::new(RefCell::new(TreeNode::new(val))));
        &mut self.right
    }

    pub fn visit(&self) -> Vec<i32> {
        self.visit_with(TraverseOrder::Preorder)
    }

    pub fn visit_with(&self, order: TraverseOrder) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];
        match order {
            TraverseOrder::Preorder => self.preorder(&mut res),
            TraverseOrder::Inorder => self.inorder(&mut res),
            TraverseOrder::Postorder => self.postorder(&mut res),
            TraverseOrder::LevelOrder => {
                let mut tmp: Vec<Vec<i32>> = vec![];
                self.levelorder(&mut tmp, 0);
                res = tmp.into_iter().flatten().collect();
            }
        }
        return res;
    }

    fn inorder_insert(self: &mut Self, v: &Vec<i32>, i: usize) {
        if 2 * i + 1 < v.len() && self.left.is_none() {
            // println!("insert lhs: {} to {}", v[2 * i + 1], 2 * i + 1);
            self.set_lhs(v[2 * i + 1]);
            let lhs: &mut TreeNode = &mut self.left.as_ref().unwrap().borrow_mut();
            lhs.inorder_insert(v, 2 * i + 1);
        }
        if 2 * i + 2 < v.len() && self.right.is_none() {
            // println!("insert rhs: {} to {}", v[2 * i + 2], 2 * i + 2);
            self.set_rhs(v[2 * i + 2]);
            let rhs: &mut TreeNode = &mut self.right.as_ref().unwrap().borrow_mut();
            rhs.inorder_insert(v, 2 * i + 2);
        }
    }

    // Traverse by different orders
    //
    fn preorder(&self, res: &mut Vec<i32>) {
        res.push(self.val);

        if self.left.is_some() {
            self.left.as_ref().unwrap().borrow().inorder(res);
        }
        if self.right.is_some() {
            self.right.as_ref().unwrap().borrow().inorder(res);
        }
    }

    fn inorder(&self, res: &mut Vec<i32>) {
        if self.left.is_some() {
            self.left.as_ref().unwrap().borrow().inorder(res);
        }
        res.push(self.val);

        if self.right.is_some() {
            self.right.as_ref().unwrap().borrow().inorder(res);
        }
    }

    fn postorder(&self, res: &mut Vec<i32>) {
        if self.left.is_some() {
            self.left.as_ref().unwrap().borrow().inorder(res);
        }
        if self.right.is_some() {
            self.right.as_ref().unwrap().borrow().inorder(res);
        }
        res.push(self.val);
    }

    fn levelorder(&self, res: &mut Vec<Vec<i32>>, level: usize) {
        if res.len() == level {
            res.push(Vec::new());
        }
        res[level].push(self.val);

        if self.left.is_some() {
            let lhs = self.left.as_ref().unwrap().borrow();
            lhs.levelorder(res, level + 1);
        }
        if self.right.is_some() {
            let rhs = self.right.as_ref().unwrap().borrow();
            rhs.levelorder(res, level + 1);
        }
    }
}

#[test]
fn run() {
    let v = vec![1, 2, 3, 4, 5];
    let t = TreeNode::from_vec(&v);
    let res1 = t.visit_with(TraverseOrder::Preorder);
    let res2 = t.visit_with(TraverseOrder::LevelOrder);

    assert_eq!(res1, vec![1, 4, 2, 5, 3]);
    assert_eq!(res2, vec![1, 2, 3, 4, 5]);
}
