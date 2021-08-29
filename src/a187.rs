// https://leetcode.com/problems/repeated-dna-sequences/

// Given a string s that represents a DNA sequence,
// return all the 10-letter-long sequences (substrings)
// that occur more than once in a DNA molecule.
// You may return the answer in any order.

// 1 <= s.length <= 10^5

#[allow(dead_code)]
pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
    use std::collections::HashSet;

    if s.len() < 10 {
        return vec![];
    }

    let mut found = HashSet::new();
    let mut repeat = HashSet::new();

    for i in 0..s.len() - 9 {
        let cur = &s[i..i + 10];

        if found.contains(cur) {
            repeat.insert(cur.to_string());
        } else {
            found.insert(cur.to_string());
        }
    }
    repeat.into_iter().collect()
}

#[test]
fn run() {
    let ans = find_repeated_dna_sequences("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT".into());
    assert_eq!(ans.len(), 2);

    let ans = find_repeated_dna_sequences("AAAAAAAAAAAAA".into());
    assert_eq!(ans.len(), 1);
}
