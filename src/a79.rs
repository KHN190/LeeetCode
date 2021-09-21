// https://leetcode.com/problems/word-search/

// Given (m, n) matrix of chars,
// return true if a given word exists in matrix.

// 1 <= m, n <= 6
// 1 <= word.length <= 15

pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
    let mut visited = vec![vec![false; board[0].len()]; board.len()];
    // use char array to process faster
    let chars: Vec<char> = word.chars().collect();

    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if search(&mut board, i, j, &chars, &mut visited) {
                return true;
            }
        }
    }
    false
}

fn search(
    board: &mut Vec<Vec<char>>,
    i: usize,
    j: usize,
    chars: &[char],
    visited: &mut Vec<Vec<bool>>,
) -> bool {
    if chars.len() == 0 {
        return true;
    }
    if i >= board.len() || j >= board[0].len() || chars[0] != board[i][j] || visited[i][j] {
        return false;
    }
    // avoid visit again
    visited[i][j] = true;
    // search remain
    let w = &chars[1..];
    let mut res = search(board, i + 1, j, w, visited) || search(board, i, j + 1, w, visited);
    if i >= 1 {
        res = res || search(board, i - 1, j, w, visited);
    }
    if j >= 1 {
        res = res || search(board, i, j - 1, w, visited);
    }
    // swap back char
    visited[i][j] = false;

    res
}

#[test]
fn run() {
    let board = vec![
        vec!['A', 'B', 'C', 'E'],
        vec!['S', 'F', 'C', 'S'],
        vec!['A', 'D', 'E', 'E'],
    ];
    let res = exist(board.clone(), "ABFC".into());
    assert!(res);

    let res = exist(board.clone(), "BFDECS".into());
    assert!(res);

    let board = vec![vec!['A'], vec!['B'], vec!['A']];
    let res = exist(board.clone(), "AABBAA".into());
    assert!(!res);
}
