// https://leetcode.com/problems/merge-k-sorted-lists/

// Merge k sorted lists to a single one.

use crate::types::ListNode;

pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut res = ListNode::new(0);
    {
        for mut node in lists {
            let l1 = node.take();
            let l2 = res.next;
            res.next = crate::a21::merge_two_lists(l1, l2);
        }
    }
    res.next
}

#[test]
fn run() {
    let l1 = vec![1, 4, 5];
    let l2 = vec![1, 3, 4];
    let l3 = vec![2, 6];

    let lists = vec![
        ListNode::from_vec(l1),
        ListNode::from_vec(l2),
        ListNode::from_vec(l3),
    ];
    let res: Vec<i32> = merge_k_lists(lists).unwrap().to_vec();
    assert_eq!(res, vec![1, 1, 2, 3, 4, 4, 5, 6]);
}
