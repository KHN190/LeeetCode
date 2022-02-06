// https://leetcode.com/problems/maximal-square/

// larger area must contain the smaller area.
// we need 2 vecs for left-top and right-bottom.

// every time we "grow", we can do row+1, or col+1
// but it asks SUQARE.

// See LC85.

pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
    let m = matrix[0].len();
    let n = matrix.len();
    let mut dp: Vec<Vec<i32>> = vec![vec![0; m]; n];
    let mut max = 0;
    for i in 0..n {
        for j in 0..m {
            if matrix[i][j] == '1' {
                let mut min = if i * j > 0 { dp[i - 1][j - 1] } else { 0 };
                if i > 0 {
                    min = min.min(dp[i - 1][j]);
                }
                if j > 0 {
                    min = min.min(dp[i][j - 1]);
                }
                dp[i][j] = min + 1;
                max = max.max(dp[i][j]);
            }
        }
    }
    max * max
}

#[test]
fn main() {
    let matrix = vec![vec!['0', '1'], vec!['1', '0']];
    let res = maximal_square(matrix);
    assert_eq!(res, 1);
}
