// https://leetcode.com/problems/maximum-subarray/

// Given an integer array nums, find the contiguous subarray
// (containing at least one number) which has the largest sum
// and return its sum.

// A subarray is a contiguous part of an array.

pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut sum: i32 = 0; // cur_sum
    let mut max: i32 = std::i32::MIN; // cur_max
    for n in nums {
        if sum <= 0 {
            sum = n;
        } else {
            sum = n + sum;
        }
        if sum > max {
            max = sum;
        }
    }
    max
}

#[test]
fn run() {
    let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    assert_eq!(max_sub_array(nums), 6);

    let nums = vec![1];
    assert_eq!(max_sub_array(nums), 1);

    let nums = vec![5, 4, -1, 7, 8];
    assert_eq!(max_sub_array(nums), 23);

    let nums = vec![-1];
    assert_eq!(max_sub_array(nums), -1);
}
