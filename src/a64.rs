// https://leetcode.com/problems/minimum-path-sum/

// Given a m x n grid filled with non-negative numbers,
// find a path from top left to bottom right,
// which minimizes the sum of all numbers along its path.
//
// You can only move either down or right at any point in time.

// for any (i, j), the minimal result is
//  min(grid[i-1,j], grid[i,j-1]) + res[i,j]

pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let m: usize = grid.len();
    let n: usize = grid[0].len();

    let mut res: Vec<Vec<i32>> = vec![vec![0; n]; m];
    for i in 0..m {
        for j in 0..n {
            if i == 0 && j == 0 {
                res[i][j] = grid[i][j];
                continue;
            }
            if i == 0 {
                res[i][j] = res[i][j - 1] + grid[i][j];
                continue;
            }
            if j == 0 {
                res[i][j] = res[i - 1][j] + grid[i][j];
                continue;
            }
            res[i][j] = res[i][j - 1].min(res[i - 1][j]) + grid[i][j];
        }
    }
    res[m - 1][n - 1]
}

#[test]
fn run() {
    let grid: Vec<Vec<i32>> = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];

    assert_eq!(min_path_sum(grid), 7);
}
