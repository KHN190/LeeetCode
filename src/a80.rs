// https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/

// array is non-decreasing.
// remove vals so they at most appear 2 times.

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.len() <= 2 {
        return nums.len() as i32;
    }
    let mut end = 2;
    for i in 2..nums.len() {
        if nums[end - 2] != nums[i] {
            nums[end] = nums[i];
            end += 1;
        }
    }
    return end as i32;
}

#[test]
fn run() {
    let mut nums: Vec<i32> = vec![1, 1, 1, 2, 2, 3, 3, 3, 4];
    assert_eq!(remove_duplicates(&mut nums), 7);
    assert_eq!(nums, vec![1, 1, 2, 2, 3, 3, 4, 3, 4]);
}
