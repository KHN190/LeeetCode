// https://leetcode.com/problems/valid-palindrome/

pub fn is_palindrome(s: String) -> bool {
    let mut simple: Vec<char> = vec![];
    for c in s.to_lowercase().chars() {
        if c.is_alphanumeric() {
            simple.push(c);
        }
    }
    is_simple_palindrome(simple.iter().collect())
}

fn is_simple_palindrome(s: String) -> bool {
    let n = s.len();
    let mut m = n / 2;
    if n % 2 == 1 {
        m = (n - 1) / 2;
    }
    let bytes = s.as_bytes();
    for i in 0..m {
        if bytes[i] != bytes[n - i - 1] {
            return false;
        }
    }
    true
}

#[test]
fn run() {
    assert_eq!(is_palindrome("abbc".into()), false);
    assert_eq!(is_palindrome("abba".into()), true);
    assert_eq!(is_palindrome("abcba".into()), true);
    assert_eq!(
        is_palindrome("A man, a plan, a canal: Panama.".into()),
        true
    );
    assert_eq!(is_palindrome("0P".into()), false);
}
