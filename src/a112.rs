use std::cell::RefCell;
use std::rc::Rc;

use crate::types::TreeNode;

pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
    if root.is_none() {
        return false;
    }
    let tmp = root.unwrap();
    let curr = tmp.borrow();
    let remain = target_sum - curr.val;
    // has children
    if curr.left.is_some() || curr.right.is_some() {
        return has_path_sum(curr.left.clone(), remain) || has_path_sum(curr.right.clone(), remain);
    }
    // no child
    remain == 0
}

#[test]
fn run() {
    //
}
