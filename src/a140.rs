// https://leetcode.com/problems/word-break-ii/

// Given a string s and a dictionary of strings wordDict,
// add spaces in s to construct a sentence where each word
// is a valid dictionary word.
// Return all such possible sentences in any order.

// the same word in the dictionary may be reused multiple times
// in the segmentation.

//
// add spaces to segment into known words
// return [] if no solution
//
// build a trie that we can search
// try on each char, try insert a space here, if it is found in trie,
// then insert. otherwise, move rightward. we also want a recursive
// function to store temp result. insert to result if we finally
// exhaust the whole string.

use crate::types::Trie;

pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
    let mut trie = Trie::new();
    for w in word_dict.iter() {
        trie.insert(w.clone());
    }

    let mut res: Vec<String> = vec![];
    recursive(&s.as_str(), &"".into(), &"".into(), &mut res, &trie);

    res
}

fn recursive(s: &str, cur_word: &String, cur: &String, res: &mut Vec<String>, trie: &Trie) {
    // add cur to res
    if s.is_empty() {
        if trie.search(cur_word.into()) {
            let new_cur = &format!("{}{} ", cur, cur_word);
            res.push(new_cur.trim().into());
        }
        return;
    }
    // if we can insert space, add current char and a space to cur_word
    // 1. trie.search
    // 2. trie.start_with
    // 3. can not find

    if trie.search(cur_word.into()) {
        let new_cur = &format!("{}{} ", cur, cur_word);
        let new_cur_word = &format!("{}{}", "", &s[0..1]);
        recursive(&s[1..s.len()], new_cur_word, new_cur, res, trie);
    }
    if !trie.starts_with(cur_word.into()) {
        return;
    }

    let new_cur_word = &format!("{}{}", cur_word, &s[0..1]);
    let new_cur = &cur.clone();
    recursive(&s[1..s.len()], new_cur_word, new_cur, res, trie);
}

#[test]
fn run() {
    let word_dict: Vec<String> = vec![
        "cat".into(),
        "cats".into(),
        "and".into(),
        "sand".into(),
        "dog".into(),
    ];
    let mut res = word_break("catsanddog".into(), word_dict);
    let mut ans: Vec<String> = vec!["cats and dog".into(), "cat sand dog".into()];
    res.sort();
    ans.sort();
    assert_eq!(res, ans);

    let word_dict: Vec<String> = vec![
        "apple".into(),
        "pen".into(),
        "applepen".into(),
        "pine".into(),
        "pineapple".into(),
    ];
    let mut res = word_break("pineapplepenapple".into(), word_dict);
    let mut ans: Vec<String> = vec![
        "pine apple pen apple".into(),
        "pineapple pen apple".into(),
        "pine applepen apple".into(),
    ];
    res.sort();
    ans.sort();
    assert_eq!(res, ans);

    let word_dict: Vec<String> = vec!["cats".into()];
    let res = word_break("catsandog".into(), word_dict);
    assert!(res.is_empty());

    let word_dict: Vec<String> = vec!["b".into()];
    let res = word_break("a".into(), word_dict);
    assert!(res.is_empty());
}
