// https://leetcode.com/problems/binary-tree-inorder-traversal/

use std::cell::RefCell;
use std::rc::Rc;

use crate::types::TreeNode;

pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = vec![];
    _traversal(&root, &mut res);
    res
}

fn _traversal(node: &Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
    if let Some(n) = node {
        _traversal(&n.borrow().left, res);
        res.push(n.borrow().val);
        _traversal(&n.borrow().right, res);
    }
}

#[test]
fn run() {
    let mut t = TreeNode::new(1);
    let mut n1 = TreeNode::new(2);
    let n2 = TreeNode::new(3);
    n1.left = Some(Rc::new(RefCell::new(n2)));
    t.right = Some(Rc::new(RefCell::new(n1)));

    let res = inorder_traversal(Some(Rc::new(RefCell::new(t))));
    assert_eq!(res, vec![1, 3, 2]);
}
