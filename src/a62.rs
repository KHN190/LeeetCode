// https://leetcode.com/problems/unique-paths/

// A robot is located at the top-left corner of a m x n grid
//
// The robot can only move either down or right at any point in time.
// The robot is trying to reach the bottom-right corner of the grid
//
// How many possible unique paths are there?

// Status: cur pos [m, n]; possible moves
// by definition it never revisit pos

pub fn unique_paths(m: i32, n: i32) -> i32 {
    // if move down, then paths(m, n - 1)
    // if move right, then paths(m - 1, n)
    if m <= 0 || n <= 0 {
        return 0;
    }
    if m == 1 && n == 1 {
        return 1;
    }
    unique_paths(m - 1, n) + unique_paths(m, n - 1)
}

#[test]
fn run() {
    assert_eq!(unique_paths(3, 2), 3);
    assert_eq!(unique_paths(3, 7), 28);
    assert_eq!(unique_paths(3, 3), 6);
}
