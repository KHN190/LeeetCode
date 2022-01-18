// https://leetcode.com/problems/kth-largest-element-in-an-array/

pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort();
    *nums.iter().rev().nth(k as usize - 1).unwrap()
}

#[test]
fn run() {
    let nums = vec![3, 2, 1, 5, 6, 4];
    assert_eq!(find_kth_largest(nums, 2), 5);
}
