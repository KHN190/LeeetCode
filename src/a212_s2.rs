// use Trie to speed up

use crate::types::Trie;

pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
    let mut res: Vec<String> = vec![];
    // bounds of the board
    let n = board.len();
    let m = board[0].len();
    let mut visited = vec![vec![false; m]; n];
    // build trie
    let mut trie = Trie::new();
    for word in &words {
        trie.insert(&word);
    }

    // for each cell, bfs to find words that starts here
    let buffer = &mut "".into();
    for i in 0..n {
        for j in 0..m {
            // recursive dfs to find word
            find_word(i, j, &board, &mut visited, buffer, &trie, &mut res);
        }
    }
    res
}

// dfs to search current word
fn find_word(
    i: usize,
    j: usize,
    board: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    buffer: &mut String, // current partial match
    trie: &Trie,
    res: &mut Vec<String>,
) {
    // if visited curr char, return.
    if visited[i][j] {
        return;
    }
    visited[i][j] = true;

    buffer.push(board[i][j]);

    // we have found the word
    if trie.search(&buffer) {
        if !res.contains(buffer) {
            res.push(buffer.to_owned());
        }
        // @notice we don't return here because
        // words can span longer
    }
    // partial match, we proceed next char
    if trie.starts_with(&buffer) {
        if i > 0 {
            find_word(i - 1, j, board, visited, buffer, &trie, res);
        }
        if i < board.len() - 1 {
            find_word(i + 1, j, board, visited, buffer, &trie, res);
        }
        if j > 0 {
            find_word(i, j - 1, board, visited, buffer, &trie, res);
        }
        if j < board[0].len() - 1 {
            find_word(i, j + 1, board, visited, buffer, &trie, res);
        }
    }

    buffer.pop();
    visited[i][j] = false;
}

#[test]
fn run() {
    let board: Vec<Vec<char>> = vec![
        vec!['o', 'a', 'a', 'n'],
        vec!['e', 't', 'a', 'e'],
        vec!['i', 'h', 'k', 'r'],
        vec!['i', 'f', 'l', 'v'],
    ];
    let words = vec![
        "oath".into(),
        "pea".into(),
        "eat".into(),
        "rain".into(),
        "oat".into(),
    ];
    let mut res = find_words(board, words);
    let mut ans: Vec<String> = vec!["eat".into(), "oath".into(), "oat".into()];
    res.sort();
    ans.sort();

    assert_eq!(res, ans);
}

#[test]
fn run2() {
    let board: Vec<Vec<char>> = vec![vec!['a']];
    let words = vec!["a".into(), "aa".into(), "b".into()];
    let res = find_words(board, words);
    let ans: Vec<String> = vec!["a".into()];

    assert_eq!(res, ans);
}

#[test]
fn run3() {
    let board: Vec<Vec<char>> = vec![vec!['a', 'a']];
    let words = vec!["aa".into()];
    let res = find_words(board, words);
    let ans: Vec<String> = vec!["aa".into()];

    assert_eq!(res, ans);
}

#[test]
fn run4() {
    let board: Vec<Vec<char>> = vec![vec!['a', 'a']];
    let words = vec!["aaa".into()];
    let res = find_words(board, words);
    let ans: Vec<String> = vec![];

    assert_eq!(res, ans);
}

#[test]
fn run5() {
    let board: Vec<Vec<char>> = vec![
        vec!['a', 'b', 'c'],
        vec!['a', 'e', 'd'],
        vec!['a', 'f', 'g'],
    ];
    let words = vec![
        "abcdefg".into(),
        "gfedcbaaa".into(),
        "eaabcdgfa".into(),
        "befa".into(),
        "dgc".into(),
        "ade".into(),
    ];
    let mut res = find_words(board, words);
    let mut ans: Vec<String> = vec![
        "abcdefg".into(),
        "befa".into(),
        "eaabcdgfa".into(),
        "gfedcbaaa".into(),
    ];
    res.sort();
    ans.sort();

    assert_eq!(res, ans);
}
