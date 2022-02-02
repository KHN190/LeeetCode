// https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/

// not a BST
//
// children of a node, where parent is n,
//  n * 2 + 1, n * 2 + 2
//
// 0 -> (1,2), 1 -> (3,4), 2 -> (5,6), 4 -> (9,10)
//
// if they are stored in array, it's much easier to
// calculate the index.

use crate::types::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

pub fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    return dfs(root.as_ref(), p.as_ref(), q.as_ref());
}

pub fn dfs(
    root: Option<&Rc<RefCell<TreeNode>>>,
    p: Option<&Rc<RefCell<TreeNode>>>,
    q: Option<&Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    // find a node that contains p or q
    if root == None || root == p || root == q {
        return root.cloned();
    }
    let lhs = dfs(root.unwrap().borrow().left.as_ref(), p, q);
    let rhs = dfs(root.unwrap().borrow().right.as_ref(), p, q);
    // if p, q on both sides
    if lhs != None && rhs != None {
        return root.cloned();
    }
    // if p, q on left side
    if lhs != None {
        return lhs;
    }
    // if p, q on right side
    if rhs != None {
        return rhs;
    }
    return None;
}
