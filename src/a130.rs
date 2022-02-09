// https://leetcode.com/problems/surrounded-regions/

// flip all 'O' not connected to edge

use std::collections::HashSet;

pub fn solve(board: &mut Vec<Vec<char>>) {
    let row = board.len();
    let col = board[0].len();
    // we store all ids that is connected to edges and is 'O'
    let mut borders: HashSet<usize> = HashSet::new();
    let mut visited: HashSet<usize> = HashSet::new();
    // so visit all border cells
    for i in 0..row {
        dfs(i, 0, row, col, board, &mut borders, &mut visited);
        dfs(i, col - 1, row, col, board, &mut borders, &mut visited);
    }
    for j in 0..col {
        dfs(0, j, row, col, board, &mut borders, &mut visited);
        dfs(row - 1, j, row, col, board, &mut borders, &mut visited);
    }
    // flip all 'O' not connected to edges
    for i in 0..row {
        for j in 0..col {
            let cur = i * col + j;
            if board[i][j] == 'O' && !borders.contains(&cur) {
                board[i][j] = 'X';
            }
        }
    }
}

fn dfs(
    i: usize,
    j: usize,
    row: usize,
    col: usize,
    board: &mut Vec<Vec<char>>,
    borders: &mut HashSet<usize>,
    visited: &mut HashSet<usize>,
) {
    // out of bound
    if i >= row || j >= col {
        return;
    }
    let cur = i * col + j;
    // skip 'X' and visited cells
    if board[i][j] == 'X' || visited.contains(&cur) {
        return;
    }
    visited.insert(cur);
    // find all connected to this cell
    if board[i][j] == 'O' {
        borders.insert(cur);
    }
    dfs(i + 1, j, row, col, board, borders, visited);
    dfs(i, j + 1, row, col, board, borders, visited);
    if i >= 1 {
        dfs(i - 1, j, row, col, board, borders, visited);
    }
    if j >= 1 {
        dfs(i, j - 1, row, col, board, borders, visited);
    }
}

#[test]
fn run() {
    let mut board: Vec<Vec<char>> = vec![
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'O', 'O', 'X'],
        vec!['X', 'X', 'O', 'X'],
        vec!['X', 'O', 'X', 'X'],
    ];
    solve(&mut board);

    let ans: Vec<Vec<char>> = vec![
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'O', 'X', 'X'],
    ];
    assert_eq!(board, ans);

    let mut board: Vec<Vec<char>> = vec![
        vec!['X', 'O', 'X', 'O', 'X', 'O'],
        vec!['O', 'X', 'O', 'X', 'O', 'X'],
        vec!['X', 'O', 'X', 'O', 'X', 'O'],
        vec!['O', 'X', 'O', 'X', 'O', 'X'],
    ];
    solve(&mut board);

    let ans: Vec<Vec<char>> = vec![
        vec!['X', 'O', 'X', 'O', 'X', 'O'],
        vec!['O', 'X', 'X', 'X', 'X', 'X'],
        vec!['X', 'X', 'X', 'X', 'X', 'O'],
        vec!['O', 'X', 'O', 'X', 'O', 'X'],
    ];
    assert_eq!(board, ans);
}
