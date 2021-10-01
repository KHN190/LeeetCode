use crate::types::ListNode;
use std::collections::BinaryHeap;

// faster than s1 O(N^2), because sort is done in one run.
pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    // sort all nodes
    let mut heap = BinaryHeap::new();
    for mut node in lists {
        if node.is_some() {
            heap.push(node.take()?)
        }
    }
    // make list from heap
    let mut head = heap.pop()?;
    let mut curr = &mut head;
    while !heap.is_empty() {
        if curr.next.is_some() {
            heap.push(curr.next.take()?);
        }
        curr.next = Some(heap.pop()?);
        curr = curr.next.as_mut()?;
    }
    Some(head)
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
