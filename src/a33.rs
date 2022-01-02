// https://leetcode.com/problems/search-in-rotated-sorted-array/

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    // O(N) solution, but we need O(log(N))
    // for (i, n) in nums.iter().enumerate()
    // {
    //     if n == &target { return i as i32; }
    // }
    // -1

    // O(log(N)) binary search
    let mut l: usize = 0;
    let mut r: usize = nums.len();

    while l < r {
        // middle pivot
        let m = (l + r) / 2;
        if target == nums[m] {
            return m as i32;
        }
        if nums[l] <= nums[m] && nums[m] <= nums[r - 1] {
            // if all in ascending order
            if target < nums[m] {
                r = m;
            } else {
                l = m + 1;
            }
        } else if nums[l] >= nums[m] {
            // if shift is on the left
            if target < nums[m] || target >= nums[l] {
                r = m;
            } else {
                l = m + 1;
            }
        } else if nums[m] >= nums[r - 1] {
            // if shift is on the right
            if target < nums[m] && target >= nums[l] {
                r = m;
            } else {
                l = m + 1;
            }
        }
    }
    -1
}

#[test]
fn run() {
    let nums: Vec<i32> = vec![4, 5, 6, 7, 0, 1, 2];
    assert_eq!(search(nums, 0), 4);

    let nums = vec![4, 5, 6, 7, 0, 1, 2];
    assert_eq!(search(nums, 3), -1);
}
