// https://leetcode.com/problems/validate-binary-search-tree/

// Given the root of a binary tree, determine if it is a valid binary search tree (BST).
// A valid BST is defined as follows:
//  * The left of a node: child.val < node.val
//  * The right of a node: child.val > node.val
//  * Both left and right must also be BST.

// Num of nodes in [1, 10^4]
// -2^31 <= Node.val <= 2^31 - 1

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
}

pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    valid(root, std::i64::MIN, std::i64::MAX)
}

fn valid(root: Option<Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
    if let Some(node) = root {
        if node.borrow().val as i64 <= min || node.borrow().val as i64 >= max {
            return false;
        }
        return valid(node.borrow().left.clone(), min, node.borrow().val as i64)
            && valid(node.borrow().right.clone(), node.borrow().val as i64, max);
    }
    // for none node, return true
    true
}
