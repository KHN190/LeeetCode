// https://leetcode.com/problems/search-a-2d-matrix/

// the matrix is actually a sorted vector but represented in 2d
//   (i, j) -> (j * width + i),
//   where i = [0, width], j = [0, height]

pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    // let v: Vec<i32> = matrix.into_iter().flatten().collect();
    let v = matrix.concat();
    match v.binary_search(&target) {
        Ok(_) => true,
        Err(_) => false,
    }
}

#[test]
fn run() {
    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6]];
    assert!(search_matrix(matrix.clone(), 5));
    assert!(!search_matrix(matrix.clone(), 10));
}
