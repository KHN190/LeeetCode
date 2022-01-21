// https://leetcode.com/problems/maximum-length-of-repeated-subarray/

// Given two integer arrays nums1 and nums2,
// return the maximum length of a subarray that appears in both arrays.

// dp solution
// https://leetcode.com/problems/longest-common-subsequence/discuss/590781/From-Brute-Force-To-DP

pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let (n, m) = (nums1.len(), nums2.len());
    // curr max match length at i, j
    // we use 1 more col and row to initialize with 0
    let mut dp: Vec<Vec<i32>> = vec![vec![0; m + 1]; n + 1];
    let mut res = 0;

    // see #1143 for difference
    for i in 1..=n {
        for j in 1..=m {
            // if curr int match, it is dp[i - 1][j - 1] + 1
            if nums1[i - 1] == nums2[j - 1] {
                dp[i][j] = 1 + dp[i - 1][j - 1];
                res = res.max(dp[i][j]);
            }
        }
    }
    res
}

#[test]
fn run() {
    let n1: Vec<i32> = vec![1, 2, 3, 2, 1];
    let n2: Vec<i32> = vec![3, 2, 1, 4, 7];
    assert_eq!(find_length(n1, n2), 3);

    let n1: Vec<i32> = vec![1, 2, 3, 0, 0];
    let n2: Vec<i32> = vec![3, 0, 0, 0, 7];
    assert_eq!(find_length(n1, n2), 3);

    let n1: Vec<i32> = vec![0, 1, 1, 1, 1];
    let n2: Vec<i32> = vec![1, 0, 1, 0, 1];
    assert_eq!(find_length(n1, n2), 2);
}
