// https://leetcode.com/problems/subarray-sum-equals-k/

// prefix sum
//
// Time Complexity:    O(n)
// Space Complexity:   O(n)

use std::collections::HashMap;

pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    let mut sum_to_freq: HashMap<i32, u32> = HashMap::new();
    let mut sum: i32 = 0;
    let mut cnt: u32 = 0;
    for num in nums {
        sum += num;
        if sum == k {
            cnt += 1;
        }
        cnt += sum_to_freq.get(&(sum - k)).unwrap_or(&0);
        *sum_to_freq.entry(sum).or_default() += 1;
    }
    cnt as i32
}

#[test]
fn run() {
    assert_eq!(subarray_sum(vec![1, 1, 1], 2), 2);
    assert_eq!(subarray_sum(vec![1, 2, 3], 3), 2);
}
