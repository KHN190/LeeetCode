// https://leetcode.com/problems/missing-number/

// only one number is missing
// variation: detect if there's a missing number,
// also do it in arithmetic way

pub fn missing_number(nums: Vec<i32>) -> i32 {
    let n: i32 = nums.len() as i32;
    (n + 1) * n / 2 - nums.iter().sum::<i32>()
}

#[test]
fn run() {
    assert_eq!(missing_number(vec![3, 0, 1]), 2);
}
