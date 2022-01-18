// binary heap solution
// it processes large data on stream

use std::collections::BinaryHeap;

pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    let mut heap = BinaryHeap::with_capacity(k as usize);
    for num in nums {
        heap.push(num);
    }
    let mut k = k;
    while k > 1 {
        heap.pop();
        k -= 1;
    }
    *heap.peek().unwrap()
}

#[test]
fn run() {
    let nums = vec![3, 2, 1, 5, 6, 4];
    assert_eq!(find_kth_largest(nums, 2), 5);
}
