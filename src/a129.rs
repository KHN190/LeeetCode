// https://leetcode.com/problems/sum-root-to-leaf-numbers/

use crate::types::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    recursive(root, 0)
}

fn recursive(node: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
    if let Some(n) = node {
        // leaf node
        if n.borrow().left.is_none() && n.borrow().right.is_none() {
            return sum * 10 + n.borrow().val;
        }
        // has left or right
        return recursive(n.borrow().left.clone(), sum * 10 + n.borrow().val)
            + recursive(n.borrow().right.clone(), sum * 10 + n.borrow().val);
    }
    return 0;
}

#[test]
fn run() {
    let v: Vec<i32> = vec![1, 2, 3];
    let t = Some(Rc::new(RefCell::new(TreeNode::from_vec(&v))));
    assert_eq!(sum_numbers(t), 25);
}
