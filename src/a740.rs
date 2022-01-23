// https://leetcode.com/problems/delete-and-earn/

// del nums[i] to earn score
// after it, del all fs equal to nums[i] - 1 or nums[i] + 1

use std::collections::{HashMap, HashSet};

pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
    // occurrences of each number, k is the n
    let mut fs = HashMap::<i32, i32>::new();
    let mut ks = HashSet::<i32>::new();
    for n in nums {
        let cnt = fs.entry(n).or_insert(0);
        *cnt += 1;
        ks.insert(n);
    }
    // min and max of all nums
    let (s1, s2) = (
        *ks.iter().min().unwrap() as usize,
        *ks.iter().max().unwrap() as usize,
    );
    // then question becomes:
    // we can earn from n in [s1..s2], but cannot rob n-1 or n+1
    let mut res: Vec<i32> = vec![];
    for (i, n) in (s1..=s2).enumerate() {
        let n = n as i32;
        let cur = n * fs.get(&n).unwrap_or(&0); // current earn if we take n

        if i >= 2 {
            res.push((res[i - 2] + cur).max(res[i - 1]));
        } else if i >= 1 {
            res.push(cur.max(res[i - 1]));
        } else {
            res.push(cur);
        }
    }
    res[res.len() - 1]
}

#[test]
fn run() {
    assert_eq!(delete_and_earn(vec![3, 4, 2]), 6);
    assert_eq!(delete_and_earn(vec![2, 2, 3, 3, 3, 4]), 9);
    assert_eq!(delete_and_earn(vec![4, 4, 2, 2]), 12);
}
