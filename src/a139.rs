// https://leetcode.com/problems/word-break/

// Given a string s, a list of words,
// true if s can be broke into combination of the words.

// 1. DFS
// 2. DP
// 3. Trie

// 1 <= s.length <= 300
// 1 <= wordDict.length <= 1000
// 1 <= wordDict[i].length <= 20

// only lowercase
// word_dict contains only unique words

// DP solution
pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    // contains whether s[..i] can be solved
    let mut res: Vec<bool> = vec![true];

    for i in 1..=s.len() {
        // any word found?
        let mut found = false;
        for w in &word_dict {
            // for current word, res[i] is based on res[i - len(w)]
            let l = w.len();
            if i >= l && res[i - l] {
                // current word exists at this position?
                let exist = w.as_str() == &s[i - l..i];
                if exist {
                    found = true;
                    break;
                }
            }
        }
        res.push(found);
    }
    res[res.len() - 1]
}

#[test]
fn run() {
    let s = String::from("applepenapple");
    let d = vec!["apple".into(), "pen".into()];
    assert!(word_break(s, d));

    let s = String::from("catsandog");
    let d = vec!["cat".into(), "sand".into(), "dog".into()];
    assert!(!word_break(s, d));
}
