// https://leetcode.com/problems/custom-sort-string/

use std::collections::HashMap;

pub fn custom_sort_string(order: String, s: String) -> String {
    let mut char_freq: HashMap<char, usize> = HashMap::new();
    for c in s.chars() {
        *char_freq.entry(c).or_insert(0) += 1;
    }
    // build the first part
    let mut res: Vec<char> = vec![];
    for c in order.chars() {
        match char_freq.get(&c) {
            Some(n) => {
                for _ in 0..*n {
                    res.push(c);
                }
                char_freq.remove(&c);
            }
            None => {}
        }
    }
    // build the rest
    for c in char_freq.keys() {
        let n = char_freq[c];
        for _ in 0..n {
            res.push(*c);
        }
    }
    res.iter().collect()
}

#[test]
fn run() {
    let order: String = "cba".into();
    let res = custom_sort_string(order, "abcd".into());
    assert_eq!(&res, &"cbad");

    let order: String = "cbafg".into();
    let res = custom_sort_string(order, "abcmm".into());
    assert_eq!(&res, &"cbamm");
}
