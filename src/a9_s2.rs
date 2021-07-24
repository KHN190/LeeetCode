// https://leetcode.com/problems/palindrome-number/

#[allow(dead_code)]
pub fn is_palindrome(x: i32) -> bool {
    // special case
    if x < 0 || (x > 0 && x % 10 == 0) {
        return false;
    }
    // reverse num
    let mut num = vec![];
    let mut n = x;

    while n > 0 {
        num.push(n % 10);
        n /= 10;
    }
    // is it palindrome?
    let mid = num.len() / 2;
    let len = num.len();

    for i in 0..mid {
        if num[i] != num[len - i - 1] {
            return false;
        }
    }
    true
}

#[test]
fn run() {
    assert_eq!(is_palindrome(10), false);
    assert_eq!(is_palindrome(11), true);
    assert_eq!(is_palindrome(121), true);
    assert_eq!(is_palindrome(5), true);
    assert_eq!(is_palindrome(-121), false);
    assert_eq!(is_palindrome(313), true);
    assert_eq!(is_palindrome(1021), false);
}
