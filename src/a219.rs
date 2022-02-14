// https://leetcode.com/problems/contains-duplicate-ii/

// return true if nums[i] == nums[j] and abs(i - j) <= k. i != j

use std::collections::{HashSet, VecDeque};

pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    let size = nums.len();
    if size <= 1 {
        return false;
    }
    // keep a set of curr n elements, n <= k
    let k = k as usize;
    let mut window = VecDeque::<i32>::with_capacity(k);
    let mut elems = HashSet::<i32>::new();
    for i in 0..size {
        // find if next elem already in the window
        if elems.contains(&nums[i]) {
            return true;
        }
        // if not, slide the window
        window.push_back(nums[i]);
        elems.insert(nums[i]);
        if window.len() > k {
            elems.remove(&window.pop_front().unwrap());
        }
    }
    false
}

#[test]
fn run() {
    let nums = vec![1, 2, 3, 1];
    assert!(contains_nearby_duplicate(nums, 3));

    let nums = vec![1, 2, 3, 1];
    assert!(!contains_nearby_duplicate(nums, 2));
}
