// https://leetcode.com/problems/binary-tree-level-order-traversal-ii/

use std::cell::RefCell;
use std::rc::Rc;

use crate::types::TreeNode;

pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = vec![];
    recursive(root.clone(), &mut res, 0);
    res.reverse();
    res
}

fn recursive(root: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<Vec<i32>>, level: usize) {
    if let Some(r) = root {
        if res.len() == level {
            res.push(Vec::new());
        }
        res[level].push(r.borrow().val);
        recursive(r.borrow().left.clone(), res, level + 1);
        recursive(r.borrow().right.clone(), res, level + 1);
    }
}

#[test]
fn run() {
    let v = vec![1, 2, 3, 4, 5];
    let t = Some(Rc::new(RefCell::new(TreeNode::from_vec(&v))));
    let res = level_order_bottom(t);
    assert_eq!(res, vec![vec![4, 5], vec![2, 3], vec![1]]);
}
