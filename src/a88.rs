// https://leetcode.com/problems/merge-sorted-array/

pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let total_len = nums1.len();
    let m = m as usize;
    let n = n as usize;

    for i in (0..m).rev() {
        nums1.swap(i, total_len - m + i);
    }

    let mut i = total_len - m;
    let mut j = 0;
    let mut k = 0;

    while i < total_len || j < n {
        if j == n || (i < total_len && nums1[i] < nums2[j]) {
            nums1.swap(k, i);
            i += 1;
        } else {
            nums1[k] = nums2[j];
            j += 1;
        }
        k += 1;
    }
}

#[test]
fn run() {
    let mut n1 = vec![1, 2, 3, 0, 0, 0];
    let mut n2 = vec![2, 5, 6];

    merge(&mut n1, 3, &mut n2, 3);
    assert_eq!(n1, vec![1, 2, 2, 3, 5, 6]);
}
