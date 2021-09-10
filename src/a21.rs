// https://leetcode.com/problems/merge-two-sorted-lists/

// Merge two sorted linked lists and return it as a sorted list.
// The list should be made by splicing together the nodes of
// the first two lists.

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn merge_two_lists(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut res = ListNode::new(0);
    {
        let mut c1 = l1;
        let mut c2 = l2;
        let mut cur = &mut res.next; // pointer to res

        // loop c1, c2
        // if c1 has no next, append c2 to res
        // if c2 has no next, append c1 to res
        // else, compare c1, c2, advance the smaller
        loop {
            if c1.is_none() {
                *cur = c2;
                break;
            } else if c2.is_none() {
                *cur = c1;
                break;
            } else {
                let v1 = get_val(&c1);
                let v2 = get_val(&c2);
                *cur = Some(Box::new(ListNode::new(v1.min(v2))));
                cur = &mut cur.as_mut().unwrap().next;

                if v1 < v2 {
                    c1 = c1.unwrap().next;
                } else {
                    c2 = c2.unwrap().next;
                }
            }
        }
    }
    res.next
}

// assume it is not none
fn get_val(l: &Option<Box<ListNode>>) -> i32 {
    l.as_ref().unwrap().as_ref().val
}
