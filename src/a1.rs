// https://leetcode.com/problems/two-sum/

// You may assume that each input would have exactly one solution,
// and you may not use the same element twice.

// 2 <= nums.length <= 10^4
// -10^9 <= nums[i] <= 10^9 -> num can be large!
// -10^9 <= target <= 10^9
// Only one valid answer exists.

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;

    let mut prev_nums = HashMap::<i32, usize>::new();
    // for each element in nums, if we can't find the other number, then
    // put this into the map, and iterate the next.
    // if we found the wanted other number, then return the pairs.
    for (i, n) in nums.iter().enumerate() {
        let remain: i32 = target - n;
        if prev_nums.contains_key(&remain) {
            return vec![prev_nums[&remain] as i32, i as i32];
        }
        prev_nums.insert(*n, i);
    }

    unreachable!("it should never reach here!");
}

#[test]
fn run() {
    let mut nums = vec![2, 7, 11, 15];
    assert!(vec_eq(two_sum(nums, 9), vec!(0, 1)));

    nums = vec![3, 2, 4];
    assert!(vec_eq(two_sum(nums, 6), vec!(1, 2)));

    nums = vec![3, 3];
    assert!(vec_eq(two_sum(nums, 6), vec!(0, 1)));
}

#[cfg(test)]
fn vec_eq(v1: Vec<i32>, v2: Vec<i32>) -> bool {
    if v1.len() != v2.len() {
        return false;
    }
    for i in 0..v1.len() {
        if v1[i] != v2[i] {
            return false;
        }
    }
    true
}
