// https://leetcode.com/problems/longest-common-subsequence/

// this is different with `substring`
// because for subsequence, "ace" is a subsequence of "abcde".

// @todo rolling hash (robin karp)

pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    let v1 = text1.as_bytes();
    let v2 = text2.as_bytes();

    let (n, m) = (v1.len(), v2.len());
    // curr max match length at i, j
    // we use 1 more col and row to initialize with 0
    let mut dp: Vec<Vec<i32>> = vec![vec![0; m + 1]; n + 1];
    let mut res = 0;

    // see #718 for difference
    for i in 1..=n {
        for j in 1..=m {
            if v1[i - 1] == v2[j - 1] {
                dp[i][j] = 1 + dp[i - 1][j - 1];
            } else {
                dp[i][j] = dp[i][j - 1].max(dp[i - 1][j]);
            }
            res = res.max(dp[i][j]);
        }
    }
    res
}

#[test]
fn run() {
    let s1: String = "abcde".into();
    let s2: String = "aabcd".into();
    assert_eq!(longest_common_subsequence(s1, s2), 4);
}
