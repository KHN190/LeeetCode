// https://leetcode.com/problems/symmetric-tree/

use std::cell::RefCell;
use std::rc::Rc;

use crate::types::TreeNode;

pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    // both some
    if let (Some(p), Some(q)) = (&p, &q) {
        // compare curr node
        if p.borrow().val == q.borrow().val {
            // compare left and right
            return is_same_tree(p.borrow().left.clone(), q.borrow().right.clone())
                && is_same_tree(p.borrow().right.clone(), q.borrow().left.clone());
        }
        return false;
    }
    // both none
    if p.is_none() && q.is_none() {
        return true;
    }
    // one is none, one is some
    false
}

pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if let Some(root) = root {
        return is_same_tree(root.borrow().left.clone(), root.borrow().right.clone());
    }
    true
}

#[test]
fn run() {
    let t = Some(Rc::new(RefCell::new(TreeNode::from_vec(&vec![1, 2, 2]))));
    assert!(is_symmetric(t));
}
