// https://leetcode.com/problems/palindrome-number/

// 0. 和 a7 解法一樣
// 1. 能用 stack 解決嗎

// run test cases
pub fn main() {
    assert_eq!(is_palindrome(10), false);
    assert_eq!(is_palindrome(11), true);
    assert_eq!(is_palindrome(121), true);
    assert_eq!(is_palindrome(5), true);
    assert_eq!(is_palindrome(-121), false);
    assert_eq!(is_palindrome(313), true);
    assert_eq!(is_palindrome(1021), false);

    println!("success.");
}

pub fn is_palindrome(x: i32) -> bool {
    // special case
    if x < 0 || (x > 0 && x % 10 == 0) {
        return false;
    }

    // get reversed number
    let rev = crate::a7::reverse(x);

    x == rev
}
