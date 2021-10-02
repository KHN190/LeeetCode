// https://leetcode.com/problems/search-a-2d-matrix-ii/

// Search num in a [M, N] matrix, true if found, else, false.
// Matrix is sorted from left to right, and top to down,
// in descending order.

// Thoughts: binary_search to determine upper bound corner

pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    // find upper bound
    let n = match matrix.binary_search_by(|x| x[0].cmp(&target)) {
        Ok(_) => return true,
        Err(x) => x,
    };

    for row in (0..n).rev() {
        let ri = matrix[row].len() - 1;

        if matrix[row][ri] < target {
            break;
        }

        match matrix.binary_search_by(|x| x[ri].cmp(&target)) {
            Ok(_) => return true,
            Err(_) => {}
        };
        match matrix[row].binary_search(&target) {
            Ok(_) => {
                return true;
            }
            Err(_) => {}
        };
    }
    false
}

#[test]
fn run() {
    let matrix = vec![vec![1, 2, 3, 4], vec![5, 6, 9, 10], vec![11, 12, 13, 14]];
    assert!(search_matrix(matrix.clone(), 6));
    assert!(!search_matrix(matrix.clone(), 7));
    assert!(!search_matrix(matrix.clone(), 8));
    assert!(search_matrix(matrix.clone(), 12));

    let matrix = vec![
        vec![4, 7, 11],
        vec![7, 11, 16],
        vec![10, 11, 20],
        vec![13, 13, 22],
    ];
    assert!(!search_matrix(matrix.clone(), 14));
    assert!(!search_matrix(matrix.clone(), 12));
    assert!(search_matrix(matrix.clone(), 11));
    assert!(!search_matrix(matrix.clone(), 1));
}
