// https://leetcode.com/problems/longest-palindromic-substring/

// Given a string s, return the longest palindromic substring in s.

// 1 <= s.length <= 1000
// s consist of only digits and English letters.

// short palindrome must be contained in long palindrome
// bin search? dp?

pub fn longest_palindrome(s: String) -> String {
    if s.len() <= 1 {
        return s;
    }

    let mut lhs: usize = 0;
    let mut rhs: usize = 0;

    let ss = s.as_bytes();

    for i in 0..s.len() {
        let (m1, n1) = expand(&ss, i, i);
        let (m2, n2) = expand(&ss, i, i + 1);
        if n1 - m1 > rhs - lhs {
            lhs = m1;
            rhs = n1;
        }
        if n2 - m2 > rhs - lhs {
            lhs = m2;
            rhs = n2;
        }
    }
    s[lhs..=rhs].into()
}

// find the longest ends for palindrome substr
// which starts from i, j
fn expand(s: &[u8], i: usize, j: usize) -> (usize, usize) {
    let mut lhs: usize = i;
    let mut rhs: usize = j;
    let mut l: usize = 0;
    let mut r: usize = 0;

    while rhs < s.len() && s[lhs] == s[rhs] {
        l = lhs;
        r = rhs;
        if lhs == 0  {
            break;
        }
        lhs -= 1;
        rhs += 1;
    }
    (l, r)
}

#[test]
fn run() {
    let s = "aba";
    assert_eq!(longest_palindrome(s.into()).as_str(), "aba");

    let s = "abac";
    assert_eq!(longest_palindrome(s.into()).as_str(), "aba");

    let s = "abba";
    assert_eq!(longest_palindrome(s.into()).as_str(), "abba");

    let s = "abcd";
    assert_eq!(longest_palindrome(s.into()).as_str(), "a");
}
