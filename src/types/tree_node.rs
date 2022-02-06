use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Default)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
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

    pub fn from_vec(v: &Vec<i32>) -> Option<Self> {
        if v.len() == 0 {
            return None;
        }
        let mut root = TreeNode::new(v[0]);
        root._insert(v, 0);
        Some(root)
    }

    pub fn set_lhs(&mut self, val: i32) -> &mut Option<Rc<RefCell<Self>>> {
        self.left = Some(Rc::new(RefCell::new(TreeNode::new(val))));
        &mut self.left
    }

    pub fn set_rhs(&mut self, val: i32) -> &mut Option<Rc<RefCell<Self>>> {
        self.right = Some(Rc::new(RefCell::new(TreeNode::new(val))));
        &mut self.right
    }

    // inorder visit
    pub fn visit(&self) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];
        self._visit(&mut res);
        return res;
    }

    fn _insert(self: &mut Self, v: &Vec<i32>, i: usize) -> &mut Self {
        if 2 * i + 1 < v.len() && self.left.is_none() {
            // println!("insert lhs: {} to {}", v[2 * i + 1], 2 * i + 1);
            self.set_lhs(v[2 * i + 1]);
            let lhs: &mut TreeNode = &mut self.left.as_ref().unwrap().borrow_mut();
            lhs._insert(v, 2 * i + 1);
        }
        if 2 * i + 2 < v.len() && self.right.is_none() {
            // println!("insert rhs: {} to {}", v[2 * i + 2], 2 * i + 2);
            self.set_rhs(v[2 * i + 2]);
            let rhs: &mut TreeNode = &mut self.right.as_ref().unwrap().borrow_mut();
            rhs._insert(v, 2 * i + 2);
        }
        self
    }

    fn _visit(&self, res: &mut Vec<i32>) {
        res.push(self.val);

        if self.left.is_some() {
            let lhs: &TreeNode = &self.left.as_ref().unwrap().borrow();
            lhs._visit(res);
        }
        if self.right.is_some() {
            let rhs: &TreeNode = &self.right.as_ref().unwrap().borrow();
            rhs._visit(res);
        }
    }
}

#[test]
fn run() {
    let v = vec![1, 2, 3, 4, 5];
    let t = TreeNode::from_vec(&v);
    let ns = t.unwrap().visit();

    assert_eq!(ns, vec![1, 2, 4, 5, 3]);
}
