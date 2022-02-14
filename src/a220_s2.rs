// 10x faster

use std::collections::BTreeSet;

pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
    if t < 0 {
        return false;
    }
    let k = k as usize;
    let t = t as i64;
    let mut bts: BTreeSet<i64> = BTreeSet::new();
    for i in 0..nums.len() {
        if i > k as usize {
            bts.remove(&(nums[i - 1 - k] as i64));
        }
        if bts
            .range(nums[i] as i64 - t..=nums[i] as i64 + t)
            .next()
            .is_some()
        {
            return true;
        }
        bts.insert(nums[i] as i64);
    }
    false
}

#[test]
fn run() {
    let nums = vec![1, 2, 3, 1];
    assert!(contains_nearby_almost_duplicate(nums, 3, 0));

    let nums = vec![1, 5, 9, 1, 5, 9];
    assert!(!contains_nearby_almost_duplicate(nums, 2, 3));

    let nums = vec![2147483640, 2147483641];
    assert!(contains_nearby_almost_duplicate(nums, 1, 100));

    let nums = vec![-2147483648, 2147483647];
    assert!(!contains_nearby_almost_duplicate(nums, 1, 1));
}
