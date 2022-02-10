// https://leetcode.com/problems/longest-consecutive-sequence/

// unordered list, find longest consecutive subarray
// [100, 99, 1, 2, 3] -> [1, 2, 3]

// https://leetcode.com/problems/longest-consecutive-sequence/discuss/41057/Simple-O(n)-with-Explanation-Just-walk-each-streak
// nums to a set, if (n - 1) not in nums, then (n) is a start
// we find longest strek from here.

// if we use an actual union find, we waste a lot of space (it's 0..MAX)

use std::collections::HashSet;

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let set: HashSet<&i32> = nums.iter().collect();
    let mut res = 0;
    for n in set.iter() {
        // if n is local min, we count streak
        if !set.contains(&(*n - 1)) {
            let mut streak = 1;
            let mut end = *n + 1;
            while set.contains(&end) {
                end += 1;
                streak += 1;
            }
            res = res.max(streak);
        }
    }
    res
}

#[test]
fn run() {
    let nums: Vec<i32> = vec![99, 0, 1, 2, 3, 4, 100];
    assert_eq!(longest_consecutive(nums), 5);
}
