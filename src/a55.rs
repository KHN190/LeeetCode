// https://leetcode.com/problems/jump-game/

// Given an integer array nums. You are initially positioned
// at the array's first index, and each element in the array
// represents your maximum jump length at that position.

// true if you can reach the last index, or false otherwise.

// Thoughts:
//  you don't have to jump exactly the num.

// 0 <= nums[i] <= 10^5, means you don't jump backwards

pub fn can_jump(nums: Vec<i32>) -> bool {
    // the max we can reach
    let mut m: i32 = 0;
    for (i, n) in nums.iter().enumerate() {
        let i = i as i32;
        // the middle can't be reached
        if i > m {
            return false;
        }
        // (n + i) is the max for nums[i]
        m = m.max(n + i);
    }
    true
}

#[test]
fn run() {
    let nums = vec![2, 3, 1, 1, 4];
    assert!(can_jump(nums));

    let nums = vec![3, 2, 1, 0, 4];
    assert!(!can_jump(nums));

    let nums = vec![2, 5, 0, 0];
    assert!(can_jump(nums));
}
