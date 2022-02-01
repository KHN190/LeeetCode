// https://leetcode.com/problems/powx-n/

pub fn my_pow(x: f64, n: i32) -> f64 {
    let mut res = 1f64;
    let (mut n, mut x) = (n, x);
    if n < 0 {
        n *= -1;
        x = 1.0 / x;
    }
    // iterative is slow! we can do recursion
    for _ in 0..n as usize {
        res *= x;
    }
    res
}

#[test]
fn run() {
    assert_eq!(my_pow(2.0, 10), 1024.0);
    assert_eq!(my_pow(2.0, -2), 0.25);
}
