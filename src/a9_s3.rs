// https://leetcode.com/problems/palindrome-number/

// generate half reversed
#[allow(dead_code)]
pub fn is_palindrome(x: i32) -> bool {
    // special case
    if x < 0 || (x > 0 && x % 10 == 0) {
        return false;
    }
    // reverse num
    let mut half: i32 = 0;
    let mut n = x;
    while n > half {
        half = half * 10 + n % 10;
        n /= 10;
    }
    n == half || n == half / 10
}
