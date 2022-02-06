// https://leetcode.com/problems/find-peak-element/

// You must write an algorithm that runs in O(log n) time.

pub fn find_peak_element(nums: Vec<i32>) -> i32 {
    let mut l = 0;
    let mut r = nums.len() - 1;

    while l < r {
        let m = (l + r) / 2;
        if nums[m] < nums[m + 1] {
            l = m + 1;
        } else {
            r = m;
        }
    }
    l as i32
}

#[test]
fn run() {
    let v: Vec<i32> = vec![1, 2, 1, 3, 5, 6, 4];
    assert_eq!(find_peak_element(v), 5);

    let v: Vec<i32> = vec![1, 2, 3, 4, 3];
    assert_eq!(find_peak_element(v), 3);

    let v: Vec<i32> = vec![3, 2];
    assert_eq!(find_peak_element(v), 0);

    let v: Vec<i32> = vec![2, 3];
    assert_eq!(find_peak_element(v), 1);
}
