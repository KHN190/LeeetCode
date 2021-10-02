// O(M + N), slower than O(log(N) * log(M))

pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let (mut i, mut j) = (0, matrix[0].len() - 1);
    while i < matrix.len() {
        match matrix[i][j].cmp(&target) {
            std::cmp::Ordering::Less => i += 1,
            std::cmp::Ordering::Equal => return true,
            std::cmp::Ordering::Greater => {
                if j > 0 {
                    j -= 1
                } else {
                    break;
                }
            }
        }
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
