// https://leetcode.com/problems/remove-duplicate-letters/
// https://leetcode.com/problems/remove-duplicate-letters/discuss/340721/Python-readable-code-with-comments

pub fn remove_duplicate_letters(s: String) -> String {
    // get last index for each char
    let mut last_idx: Vec<Option<usize>> = vec![None; 26];
    for (i, c) in s.chars().enumerate() {
        let n = c as usize - 'a' as usize;
        last_idx[n] = Some(i);
    }
    // build temp result
    let mut res: Vec<char> = vec![];
    for (i, c) in s.chars().enumerate() {
        // when a char is unseen
        if !res.contains(&c) {
            // if it's smaller than <the last stack char>
            // and if i is not the last idx of <the last stack char>
            while let Some(&last) = res.last() {
                if c < last && i < last_idx[char2u(last)].unwrap() {
                    res.pop();
                    continue;
                }
                break;
            }
            res.push(c);
        }
    }
    res.iter().collect()
}

#[inline]
fn char2u(c: char) -> usize {
    c as usize - 'a' as usize
}

#[test]
fn run() {
    assert_eq!(&remove_duplicate_letters("bcabc".into()), "abc");
    assert_eq!(&remove_duplicate_letters("cbacdcbc".into()), "acdb");
}
