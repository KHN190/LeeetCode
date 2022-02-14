// https://leetcode.com/problems/contains-duplicate-iii/

// true if abs(nums[i] - nums[j]) <= t, abs(i - j) <= k. i != j.

// maintain a window of curr nums, and the smallest diff
// among all window nums. to make the diff also sliding,
// we need to find the closest n to the new added num. so
// we sort the curr window and do binary search.

use std::collections::VecDeque;

pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
    let k = k as usize;
    let mut window = VecDeque::<i32>::with_capacity(k);
    let mut sorted = Vec::<i32>::with_capacity(k);
    // create the window
    for n in nums.iter() {
        if sorted.len() > 0 {
            // find closest number
            match sorted.binary_search(&n) {
                Ok(_) => {
                    // we have a dup num already, diff is 0
                    return true;
                }
                Err(i) => {
                    // diff < t, return true
                    if close_to(&sorted, i, n, t) || close_to(&sorted, i - 1, n, t) {
                        return true;
                    }
                }
            };
        }
        // otherwise, we haven't found it, slide the window
        window.push_back(*n);
        sorted.push(*n);
        sorted.sort();

        if window.len() > k {
            // remove first elem from window,
            // remove it from sorted vec too.
            let old = window.pop_front().unwrap();
            if let Ok(i) = sorted.binary_search(&old) {
                sorted.remove(i);
            }
        }
    }
    false
}

#[inline]
fn close_to(sorted: &Vec<i32>, i: usize, n: &i32, t: i32) -> bool {
    if i < sorted.len() {
        // handle overflow
        let diff = (sorted[i] as i64 - *n as i64).abs();
        if diff <= t as i64 {
            return true;
        }
    }
    false
}

#[test]
fn run() {
    // let nums = vec![1, 2, 3, 1];
    // assert!(contains_nearby_almost_duplicate(nums, 3, 0));
    //
    // let nums = vec![1, 5, 9, 1, 5, 9];
    // assert!(!contains_nearby_almost_duplicate(nums, 2, 3));

    let nums = vec![2147483640, 2147483641];
    assert!(contains_nearby_almost_duplicate(nums, 1, 100));

    let nums = vec![-2147483648, 2147483647];
    assert!(!contains_nearby_almost_duplicate(nums, 1, 1));
}
