// https://leetcode.com/problems/group-anagrams/

use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map = HashMap::<String, Vec<String>>::new();
    for s in strs {
        let w = s.clone();
        let mut chrs: Vec<char> = w.chars().collect();
        chrs.sort();

        let k: String = chrs.into_iter().collect();
        map.entry(k).or_insert(Vec::<String>::new()).push(w);
    }
    map.into_values().collect()
}

#[test]
fn run() {
    let strs: Vec<String> = vec![
        "eat".into(),
        "tea".into(),
        "tan".into(),
        "ate".into(),
        "nat".into(),
        "bat".into(),
    ];
    let res = group_anagrams(strs);

    assert_eq!(res.len(), 3);
}
