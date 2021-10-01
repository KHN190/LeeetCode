use std::cmp::Ordering;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn from_vec(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut res = ListNode::new(0);
        let mut cur = &mut res.next;
        for x in vec {
            *cur = Some(Box::new(ListNode::new(x)));
            cur = &mut cur.as_mut().unwrap().next;
        }
        res.next
    }

    pub fn to_vec(&self) -> Vec<i32> {
        let mut res = vec![self.val];
        let mut cur = &self.next;
        while !cur.is_none() {
            res.push(cur.clone().unwrap().val);
            cur = &cur.as_ref().unwrap().next;
        }
        res
    }
}

impl PartialOrd<ListNode> for ListNode {
    fn partial_cmp(&self, other: &ListNode) -> Option<Ordering> {
        other.val.partial_cmp(&self.val)
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.val.cmp(&self.val)
    }
}
