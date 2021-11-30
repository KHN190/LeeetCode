// https://leetcode.com/problems/longest-substring-without-repeating-characters/

use std::collections::HashSet;

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut set: HashSet<u8> = HashSet::new();
    let mut res: i32 = 0;
    let mut lhs: usize = 0;
    let ss = s.as_bytes();
    for c in ss {
        if set.contains(&c) {
            // move to the current char index
            // when the char is duplicated
            // and remove the chars before the dup char
            loop {
                let cur = &ss[lhs];
                set.remove(&cur);
                lhs += 1;
                if cur == c {
                    break;
                }
            }
        }
        set.insert(*c);
        res = res.max(set.len() as i32);
    }
    res
}

#[test]
fn run() {
    assert_eq!(length_of_longest_substring("abcd".into()), 4);
    assert_eq!(length_of_longest_substring("pwwkew".into()), 3);
    assert_eq!(length_of_longest_substring("bbbbb".into()), 1);
    assert_eq!(length_of_longest_substring("dvdf".into()), 3);
    assert_eq!(length_of_longest_substring("aabaab!bb".into()), 3);
}
