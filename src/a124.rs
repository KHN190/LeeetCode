// https://leetcode.com/problems/binary-tree-maximum-path-sum/

// it's similar to find a max sum window in array
// but each node has at most 3 adjacencies (parent, 2 children)
//
// need to know:
// current path, current sum and visited nodes.
//
// start from root, recursively do the same for children
// max sum path = include cur node if cur.val >= 0
// include lhs if lhs >= 0, same for rhs
//
// otherwise, choose from lhs, rhs which has greater sum
//
// termination: when lhs, cur, rhs are all visited.
// (lhs == null, rhs == null)
//
// if cur node has a parent in path, it can't go to both children
// the search needs to be DFS

use crate::types::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut res = i32::MIN;
    path_sum(&root, &mut res);
    res
}

fn path_sum(node: &Option<Rc<RefCell<TreeNode>>>, cur_sum: &mut i32) -> i32 {
    if let Some(n) = node {
        let cur = n.borrow().val;
        // max sum of lhs & rhs
        let lhs = path_sum(&n.borrow().left, cur_sum);
        let rhs = path_sum(&n.borrow().right, cur_sum);
        // include cur if > cur_sum
        *cur_sum = *cur_sum.max(&mut (cur + lhs + rhs));
        return cur + lhs.max(rhs);
    }
    0
}

#[test]
fn run() {
    let mut tree = TreeNode::new(-2);
    tree.set_lhs(-1);
    let root = Some(Rc::new(RefCell::new(tree)));
    assert_eq!(max_path_sum(root), -1);

    let mut tree = TreeNode::new(1);
    tree.set_lhs(2);
    tree.set_rhs(3);

    let root = Some(Rc::new(RefCell::new(tree)));
    assert_eq!(max_path_sum(root), 6);

    let mut tree = TreeNode::new(-10);
    tree.set_lhs(9);
    tree.set_rhs(20);

    let root = Some(Rc::new(RefCell::new(tree)));
    assert_eq!(max_path_sum(root), 20);

    let mut tree = TreeNode::new(-10);
    tree.set_lhs(9);
    tree.set_rhs(20);
    let mut rhs = TreeNode::new(20);
    rhs.set_lhs(15);
    rhs.set_rhs(7);
    tree.right = Some(Rc::new(RefCell::new(rhs)));

    let root = Some(Rc::new(RefCell::new(tree)));
    assert_eq!(max_path_sum(root), 42);
}
