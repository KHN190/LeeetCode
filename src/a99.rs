// https://leetcode.com/problems/recover-binary-search-tree/

use std::cell::RefCell;
use std::rc::Rc;

use crate::types::TreeNode;

// https://leetcode.com/problems/recover-binary-search-tree/discuss/32535/No-Fancy-Algorithm-just-Simple-and-Powerful-In-Order-Traversal

type Node = Option<Rc<RefCell<TreeNode>>>;

pub fn recover_tree(root: &mut Node) {
    let mut n1: Node = None;
    let mut n2: Node = None;
    let mut prev: Node = None;
    traverse(root, &mut n1, &mut n2, &mut prev);
    // swap n1, n2
    std::mem::swap(
        &mut n1.unwrap().borrow_mut().val,
        &mut n2.unwrap().borrow_mut().val,
    );
}

fn traverse(node: &Node, n1: &mut Node, n2: &mut Node, prev: &mut Node) {
    // ignore none, only proceed if node is some
    if let Some(curr_n) = node {
        let curr = curr_n.borrow();
        // inorder: visit left first
        traverse(&curr.left.clone(), n1, n2, prev);
        // inorder: visit curr then
        if let Some(prev_n) = prev {
            if prev_n.borrow().val >= curr.val {
                // find first node to swap
                if n1.is_none() {
                    *n1 = Some(prev_n.clone());
                }
                // find second node to swap
                if n1.is_some() {
                    *n2 = Some(curr_n.clone());
                }
            }
        }
        *prev = Some(curr_n.clone());
        // inorder: visit right last
        traverse(&curr.right.clone(), n1, n2, prev);
    }
}
