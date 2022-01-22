// https://leetcode.com/problems/rotate-image/

// rotate the matrix in-place

// ~~calculate the index mapping~~
// transpose + swap opposite

pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    // 7 8 9
    // 4 5 6
    // 1 2 3
    matrix.reverse();

    for i in 0..matrix.len() {
        for j in 0..i {
            let tmp = matrix[i][j];
            matrix[i][j] = matrix[j][i];
            matrix[j][i] = tmp;
        }
    }
}

#[test]
fn run() {
    let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    rotate(&mut matrix);
    let ans = vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]];
    assert_eq!(matrix, ans);
}
