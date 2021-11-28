use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
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

    pub fn set_lhs(&mut self, val: i32) -> &mut Self {
        self.left = Some(Rc::new(RefCell::new(TreeNode::new(val))));
        self
    }

    pub fn set_rhs(&mut self, val: i32) -> &mut Self {
        self.right = Some(Rc::new(RefCell::new(TreeNode::new(val))));
        self
    }
}
