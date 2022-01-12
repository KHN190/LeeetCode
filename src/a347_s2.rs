// quick select solution

use std::cmp::Ordering;
use std::collections::HashMap;

fn partition(slice: &mut [(i32, usize)], left: usize, right: usize) -> usize {
    let pivot = left + (right - left) / 2;
    slice.swap(pivot, right);

    let mut i = left;
    for j in left..right {
        if slice[j].1 > slice[right].1 {
            slice.swap(i, j);
            i += 1;
        }
    }

    slice.swap(i, right);
    i
}

fn quick_select(slice: &mut [(i32, usize)], mut left: usize, mut right: usize, k: usize) {
    while left < right {
        let mid = partition(slice, left, right);

        match k.cmp(&mid) {
            Ordering::Greater => left = mid + 1,
            Ordering::Equal => break,
            Ordering::Less => right = mid - 1,
        }
    }
}

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

    quick_select(&mut freq, 0, len - 1, k);
    // freq.sort_by(|a, b| b.1.cmp(&a.1));

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
