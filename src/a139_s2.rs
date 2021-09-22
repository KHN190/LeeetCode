// word break DFS solution
use std::collections::HashSet;

pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    let mut failed = HashSet::<usize>::new();
    dfs(&s, 0, &word_dict, &mut failed)
}

fn dfs(s: &String, start: usize, word_dict: &Vec<String>, failed: &mut HashSet<usize>) -> bool {
    // string end reached, we are done
    if start == s.len() {
        return true;
    }
    for i in start + 1..=s.len() {
        if failed.contains(&i) {
            continue;
        }
        // word dict contains cur word?
        if contains(word_dict, &s[start..i], i - start) {
            // if so, search next
            if dfs(s, i, word_dict, failed) {
                return true;
            }
            // otherwise, it's known not working
            failed.insert(i);
        }
    }
    false
}

fn contains(word_dict: &Vec<String>, cur: &str, l: usize) -> bool {
    for w in word_dict {
        if l >= w.len() {
            if w.as_str() == cur {
                return true;
            }
        }
    }
    false
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
