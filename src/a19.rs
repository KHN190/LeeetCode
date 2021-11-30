// https://leetcode.com/problems/remove-nth-node-from-end-of-list/

// use two pointers, one on the rhs, one is n step on the lhs

use crate::types::ListNode;

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode {
        val: -1,
        next: head,
    });

    let mut rhs = dummy.clone();
    let mut lhs = dummy.as_mut();

    for _ in 0..n {
        rhs = rhs.next.unwrap();
    }

    while let Some(node) = rhs.next {
        rhs = node;
        lhs = lhs.next.as_mut().unwrap();
    }
    lhs.next = lhs.next.as_mut().unwrap().next.clone();

    dummy.next
}
