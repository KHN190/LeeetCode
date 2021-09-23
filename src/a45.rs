// https://leetcode.com/problems/jump-game-ii/

// Given int array nums, start with index 0.
// nums[i] is the max jump at the position.
// Reach the end with min jumps.

// You can assume that you can always reach the last index.

pub fn jump(nums: Vec<i32>) -> i32 {
    let (mut steps, mut m, mut end) = (0, 0, 0);
    for i in 0..nums.len() - 1 {
        // cur max reach
        m = m.max(i as i32 + nums[i]);
        // cur end needs to extend?
        if i == end {
            steps += 1;
            end = m as usize;
        }
    }
    steps
}

#[test]
fn run() {
    assert_eq!(jump(vec![2, 3, 1, 1, 4]), 2);
    assert_eq!(jump(vec![2, 3, 0, 1, 4]), 2);
    assert_eq!(jump(vec![1, 1, 1, 1, 1]), 4);
    assert_eq!(jump(vec![2, 3, 1, 1, 4, 3]), 3);
}
