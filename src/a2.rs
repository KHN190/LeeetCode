// https://leetcode.com/problems/add-two-numbers/

// You are given two non-empty linked lists representing two non-negative integers.
// The digits are stored in reverse order, and each of their nodes contains
// a single digit.

// Add the two numbers and return the sum as a linked list.

// You may assume the two numbers do not contain any leading zero,
// except the number 0 itself.

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

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut res = ListNode::new(0);
    {
        let mut c1 = l1;
        let mut c2 = l2;
        let mut cur = &mut res.next; // pointer to res
        let mut carry = 0i32;

        // if c1 has no next, append rest + carry to result
        // if c2 has no next, append rest + carry to result
        // else advance both, store cur & carry
        loop {
            if c1.is_none() && c2.is_none() {
                break;
            }
            // current values
            let v1 = if !c1.is_none() {
                c1.as_ref().unwrap().val
            } else {
                0
            };
            let v2 = if !c2.is_none() {
                c2.as_ref().unwrap().val
            } else {
                0
            };
            // calc digit and carry
            let mut val = v1 + v2 + carry;
            if val >= 10 {
                val -= 10;
                carry = 1;
            } else {
                carry = 0;
            }
            // digit to result
            *cur = Some(Box::new(ListNode::new(val)));
            cur = &mut cur.as_mut().unwrap().next;

            // advance lists
            if !c1.is_none() {
                c1 = c1.unwrap().next;
            }
            if !c2.is_none() {
                c2 = c2.unwrap().next;
            }
        }
        if carry != 0 {
            *cur = Some(Box::new(ListNode::new(carry)));
        }
    }

    res.next
}
