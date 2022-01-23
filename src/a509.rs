pub fn fib(n: i32) -> i32 {
    let mut fibs = [0i32; 31];
    fibs[0] = 0;
    fibs[1] = 1;

    for i in 2..=n as usize {
        fibs[i] = fibs[i - 1] + fibs[i - 2];
    }
    fibs[n as usize]
}

#[test]
fn run() {
    assert_eq!(fib(2), 1);
    assert_eq!(fib(3), 2);
    assert_eq!(fib(4), 3);
}
