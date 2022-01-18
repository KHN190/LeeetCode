// quick select solution (non built-in)

use crate::knife::quick_select;

pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let len = nums.len();

    quick_select(&mut nums, 0, len - 1, k);

    *nums.iter().nth(k - 1).unwrap()
}

#[test]
fn run() {
    let nums = vec![3, 2, 1, 5, 6, 4];
    assert_eq!(find_kth_largest(nums, 2), 5);

    let nums = vec![5, 2, 4, 1, 3, 6, 0];
    assert_eq!(find_kth_largest(nums, 2), 5);
}
