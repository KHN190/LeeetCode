// https://leetcode.com/problems/top-k-frequent-elements/

// Given an integer array nums and an integer k, return the k most
// frequent elements. You may return the answer in any order.

// 1 <= nums.length <= 10^5
// It is guaranteed that the answer is unique.

// Follow up: time must be O(n log n)

// Need: n + freq => HashTable / Heap
// Monotonic queue? (VecDeque)

use crate::types::Elem;

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut topk = Vec::<Elem>::new();
    for n in nums {
        // this search is slow.
        let r = topk.iter().position(|e| e.val == n);
        match r {
            Some(ix) => topk[ix].prior += 1,
            _ => topk.push(Elem::new(n, 1)),
        };
        topk.sort_by(|a, b| a.prior.cmp(&b.prior));
    }
    topk.iter().rev().take(k as usize).map(|e| e.val).collect()
}

#[test]
fn run() {
    let nums = vec![1, 0, 1, 1, 1, 2, 2, 2, 3, 3];
    assert_eq!(top_k_frequent(nums, 2), vec![1, 2]);

    let nums = vec![1, 1, 1, 2, 2, 3];
    assert_eq!(top_k_frequent(nums, 2), vec![1, 2]);
}
