// https://leetcode.com/problems/two-sum/

// You may assume that each input would have exactly one solution,
// and you may not use the same element twice.

// 2 <= nums.length <= 10^4
// -10^9 <= nums[i] <= 10^9 -> num can be large!
// -10^9 <= target <= 10^9
// Only one valid answer exists.

// 1. 為什麼要用 HashMap? 為什麼用了就可以減少時間？
//    在找到另一個數的時候，把遍歷的 O(n) 簡化為查詢 O(1) ->
//    實際上 HashSet 就可以，但我們進一步需要 index 所以用 HashMap
//
// 2. *如果不止一對，要返回所有的 index，怎麼辦？
// 3. *如果不止一對，要返回 index 的數量，怎麼辦？

// run test cases
pub fn main() {
    let mut nums = vec![2, 7, 11, 15];
    assert!(vec_eq(two_sum(nums, 9), vec!(0, 1)));

    nums = vec![3, 2, 4];
    assert!(vec_eq(two_sum(nums, 6), vec!(1, 2)));

    nums = vec![3, 3];
    assert!(vec_eq(two_sum(nums, 6), vec!(0, 1)));

    println!("success.");
}

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
