// https://leetcode.com/problems/valid-anagram/

pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let mut sf = [0; 26];
    let mut tf = [0; 26];

    for c in s.chars() {
        let i: usize = c as usize - 'a' as usize;
        sf[i] += 1;
    }
    for c in t.chars() {
        let i: usize = c as usize - 'a' as usize;
        tf[i] += 1;
    }
    for i in 0..26 {
        if sf[i] != tf[i] {
            return false;
        }
    }
    true
}

#[test]
fn run() {
    assert_eq!(is_anagram("abcd".into(), "adcb".into()), true);
    assert_eq!(is_anagram("abe".into(), "adc".into()), false);
}
