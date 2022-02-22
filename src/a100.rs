// https://leetcode.com/problems/same-tree/

use std::cell::RefCell;
use std::rc::Rc;

use crate::types::TreeNode;

type Node = Option<Rc<RefCell<TreeNode>>>;

pub fn is_same_tree(p: Node, q: Node) -> bool {
    match (p, q) {
        (None, None) => true,
        (Some(p), Some(q)) => {
            let p = p.borrow();
            let q = q.borrow();
            p.val == q.val
                && is_same_tree(p.left.clone(), q.left.clone())
                && is_same_tree(p.right.clone(), q.right.clone())
        }
        _ => false,
    }
}
