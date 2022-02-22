// https://leetcode.com/problems/balanced-binary-tree/

use std::cell::RefCell;
use std::rc::Rc;

use crate::types::TreeNode;

// visit all leaves, update min & max height, count them in the end

type Node = Option<Rc<RefCell<TreeNode>>>;

pub fn is_balanced(root: Node) -> bool {
    balanced(root).is_ok()
}

// input: tree and current height
fn balanced(node: Node) -> Result<i32, ()> {
    if let Some(n) = node {
        let curr = n.borrow();
        let l = balanced(curr.left.clone())?;
        let r = balanced(curr.right.clone())?;
        let level = l.max(r);
        if (l - r).abs() > 1 {
            Err(())
        } else {
            Ok(1 + level)
        }
    } else {
        Ok(0)
    }
}
