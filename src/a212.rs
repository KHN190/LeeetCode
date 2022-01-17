// https://leetcode.com/problems/word-search-ii/

// find all words in the given dict
// solution: build trie using the matrix, the match words

pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
    let mut res: Vec<String> = vec![];
    // bounds of the board
    let n = board.len();
    let m = board[0].len();
    let mut visited = vec![vec![false; m]; n];

    // for each cell, dfs to find words that starts here
    for i in 0..n {
        for j in 0..m {
            for word in &words {
                // skip word that already found
                if res.contains(word) {
                    continue;
                }
                // recursive dfs to find word
                find_word(
                    i,
                    j,
                    n,
                    m,
                    &board,
                    &mut visited,
                    &"".into(),
                    &word.as_str(),
                    &mut res,
                );
            }
        }
    }
    res
}

// dfs to search current word
// @notice it's dfs not bfs
fn find_word(
    i: usize,
    j: usize,
    n: usize,
    m: usize,
    board: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    cur: &String, // current partial match
    remain: &str, // remained chars
    res: &mut Vec<String>,
) {
    // we have found the word
    if remain.is_empty() {
        if !res.contains(cur) {
            res.push(cur.clone());
        }
        return;
    }
    // if visited curr char, return.
    if visited[i][j] {
        return;
    }
    visited[i][j] = true;

    // if curr char not match , return and do nothing
    let cur_char = remain.chars().nth(0).unwrap();
    if cur_char != board[i][j] {
        visited[i][j] = false;
        return;
    }
    // check next char until word is emptied.
    // concat current char to cur
    let mut cur_word = cur.clone();
    cur_word.push(cur_char);

    if i > 0 {
        find_word(
            i - 1,
            j,
            n,
            m,
            board,
            visited,
            &cur_word,
            &remain[1..remain.len()],
            res,
        );
    }
    if i < n - 1 {
        find_word(
            i + 1,
            j,
            n,
            m,
            board,
            visited,
            &cur_word,
            &remain[1..remain.len()],
            res,
        );
    }
    if j > 0 {
        find_word(
            i,
            j - 1,
            n,
            m,
            board,
            visited,
            &cur_word,
            &remain[1..remain.len()],
            res,
        );
    }
    if j < m - 1 {
        find_word(
            i,
            j + 1,
            n,
            m,
            board,
            visited,
            &cur_word,
            &remain[1..remain.len()],
            res,
        );
    }

    // handle single cell matrix
    find_word(
        i,
        j,
        n,
        m,
        board,
        visited,
        &cur_word,
        &remain[1..remain.len()],
        res,
    );

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
