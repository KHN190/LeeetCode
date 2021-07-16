// https://leetcode.com/problems/reverse-integer/

// Assume the environment does not allow you to store 64-bit integers (signed or unsigned).
// -231 <= x <= 231 - 1

// 考點在於 overflow 的處理。

#[test]
fn run_test() {
    assert_eq!(reverse(123), 321);
    assert_eq!(reverse(-123), -321);
    assert_eq!(reverse(120), 21);
    assert_eq!(reverse(0), 0);
    assert_eq!(reverse(1534236469), 0);
}

pub fn reverse(x: i32) -> i32 {
    // special case: 0
    if x == 0 {
        return x;
    }

    // keep the sign
    let sign = if x >= 0 { 1 } else { -1 };
    // unsign the num
    let mut num = if x >= 0 { x } else { -x };
    // reverse the unsigned
    let mut result: i32 = 0;

    while num > 0 {
        let curr = num % 10;
        // if goes out of bound, return 0
        // otherwise just as is
        let res = result.checked_mul(10).and_then(|x| x.checked_add(curr));
        if res.is_none() {
            return 0;
        }

        result = res.unwrap();
        num = num / 10;
    }

    result.checked_mul(sign).unwrap_or(0)
}
