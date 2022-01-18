// quick select solution

use std::collections::HashMap;

use crate::knife::quick_select_by_freq;

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    // create freq hashmap
    let freq = nums.into_iter().fold(HashMap::new(), |mut hm, n| {
        *hm.entry(n).or_insert(0) += 1;
        hm
    });
    // sort by freq
    let mut freq: Vec<(i32, usize)> = freq.into_iter().collect();
    let len = freq.len();

    quick_select_by_freq(&mut freq, 0, len - 1, k);

    freq.into_iter().map(|e| e.0).take(k as usize).collect()
}

#[test]
fn run() {
    let nums = vec![1, 0, 1, 1, 1, 2, 2, 2, 3, 3];
    let mut res = top_k_frequent(nums, 2);
    res.sort();

    assert_eq!(res, vec![1, 2]);

    let nums = vec![1, 1, 1, 2, 2, 3];
    let mut res = top_k_frequent(nums, 2);
    res.sort();

    assert_eq!(res, vec![1, 2]);
}
