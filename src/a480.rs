// https://leetcode.com/problems/sliding-window-median/
//
// Related:
//       Find Median from Data Stream
//   https://leetcode.com/problems/find-median-from-data-stream/

// 1. window size 1 <= k <= nums.length <= 10^5
// 2. 2^31 <= nums[i] <= 2^31 - 1

use std::cmp::Reverse; // MaxHeap
use std::collections::BinaryHeap; // for MinHeap

#[test]
fn run() {
    // test different windows
    let nums = vec![1, 2, 3, 4, 5, 1, 2];
    assert_eq!(
        median_sliding_window(nums.clone(), 3),
        vec!(2.0, 3.0, 4.0, 4.0, 2.0)
    );
    assert_eq!(
        median_sliding_window(nums.clone(), 4),
        vec!(2.5, 3.5, 3.5, 3.0)
    );

    // test delete only once
    let nums = vec![1, 2, 3, 4, 2, 3, 1, 4, 2];
    assert_eq!(
        median_sliding_window(nums.clone(), 3),
        vec!(2.0, 3.0, 3.0, 3.0, 2.0, 3.0, 2.0)
    );

    // test overflow
    let nums = vec![2147483647, 2147483647];
    assert_eq!(median_sliding_window(nums.clone(), 2), vec!(2147483647.0));
}

#[allow(dead_code)]
pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
    let mut lhs = BinaryHeap::<i32>::new();
    let mut rhs = BinaryHeap::<Reverse<i32>>::new();

    let mut res: Vec<f64> = vec![];
    // insert all numbers, when exceeds the lhs limit,
    // move it to rhs.
    for (ix, i) in nums.iter().enumerate() {
        // which side should i go?
        if rhs.is_empty() || (!rhs.is_empty() && rhs_top(&rhs) < *i) {
            rhs.push(Reverse(*i));
        } else {
            lhs.push(*i);
        }

        balance(&mut lhs, &mut rhs);

        // remove oldest elem
        let total = (rhs.len() + lhs.len()) as i32;
        if total > k {
            let oldest = nums[ix - k as usize];
            // remove oldest
            if oldest >= rhs_top(&rhs) {
                rhs = rebuild_rhs_without(rhs, oldest);
            } else {
                lhs = rebuild_lhs_without(lhs, oldest);
            }

            balance(&mut lhs, &mut rhs);
        }
        // we return the median!
        if total >= k {
            let median = if k % 2 == 1 {
                rhs_top(&rhs) as f64
            } else {
                // corner case: overflow!
                lhs_top(&lhs) as f64 * 0.5 + rhs_top(&rhs) as f64 * 0.5
            };
            res.push(median);
        }
    }
    res
}

// remove an element and rebuild heap
fn rebuild_lhs_without(lhs: BinaryHeap<i32>, x: i32) -> BinaryHeap<i32> {
    let mut heap = BinaryHeap::<i32>::new();
    let mut removed_once = false;
    for c in lhs.into_sorted_vec() {
        if c == x && !removed_once {
            removed_once = true;
        } else {
            heap.push(c);
        }
    }
    heap
}
fn rebuild_rhs_without(rhs: BinaryHeap<Reverse<i32>>, x: i32) -> BinaryHeap<Reverse<i32>> {
    let mut heap = BinaryHeap::<Reverse<i32>>::new();
    let mut removed_once = false;
    for c in rhs.into_sorted_vec() {
        if c.0 == x && !removed_once {
            removed_once = true;
        } else {
            heap.push(c);
        }
    }
    heap
}
// rhs always has 1 more element
fn balance(lhs: &mut BinaryHeap<i32>, rhs: &mut BinaryHeap<Reverse<i32>>) {
    let s1 = lhs.len() as i32;
    let s2 = rhs.len() as i32;
    if s1 - s2 >= 1 {
        rhs.push(Reverse(lhs.pop().unwrap()));
    }
    if s2 - s1 > 1 {
        lhs.push(rhs.pop().unwrap().0);
    }
}
// peek the top
fn lhs_top(lhs: &BinaryHeap<i32>) -> i32 {
    *lhs.peek().unwrap()
}
fn rhs_top(rhs: &BinaryHeap<Reverse<i32>>) -> i32 {
    rhs.peek().unwrap().0
}
