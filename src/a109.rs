// https://leetcode.com/problems/convert-sorted-list-to-binary-search-tree/
// https://leetcode.com/problems/convert-sorted-list-to-binary-search-tree/discuss/1194315/Rust-recursive-solution

// N + log(N)
// another approach is using slow and fast header,
// fast is 2x moving speed of slow, so they have slow & fast as tree's tail.

use std::cell::RefCell;
use std::rc::Rc;

use crate::types::{ListNode, TreeNode};

pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut len = 0;
    let mut node = &head;
    while let Some(n) = node {
        len += 1;
        node = &n.next;
    }
    let mut head = head;
    recursive(&mut head, len)
}

// recursively cut list and build the tree
fn recursive(list: &mut Option<Box<ListNode>>, len: i32) -> Option<Rc<RefCell<TreeNode>>> {
    if len == 0 {
        return None;
    }
    // this will cut the list to len / 2
    let left = recursive(list, len / 2);
    // create tree
    if let Some(head) = list {
        let mut node = TreeNode::new(head.val);
        // cut left sub list
        *list = head.next.take();
        // assign tree lhs, rhs
        node.left = left;
        node.right = recursive(list, len - len / 2 - 1);
        return Some(Rc::new(RefCell::new(node)));
    }
    None
}
