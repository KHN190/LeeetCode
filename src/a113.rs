// https://leetcode.com/problems/path-sum-ii/

use std::cell::RefCell;
use std::rc::Rc;

use crate::types::TreeNode;

type Node = Option<Rc<RefCell<TreeNode>>>;

pub fn path_sum(root: Node, target_sum: i32) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = vec![];
    recursive(root, target_sum, &mut vec![], &mut res);
    res
}

// stack contains curr path values
fn recursive(node: Node, remain: i32, stack: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
    if let Some(tmp) = node {
        let curr = tmp.borrow();
        let remain = remain - curr.val;

        stack.push(curr.val);
        // has >= 1 child(ren)
        if curr.left.is_some() || curr.right.is_some() {
            recursive(curr.left.clone(), remain, stack, res);
            recursive(curr.right.clone(), remain, stack, res);
        } else {
            // otherwise, we have found a result!
            if remain == 0 {
                res.push(stack.clone());
            }
        }
        // after visiting curr node, we pop the curr node's val
        stack.pop();
    }
}
