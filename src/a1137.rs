// https://leetcode.com/problems/n-th-tribonacci-number/

pub fn tribonacci(n: i32) -> i32 {
    let mut tribs = [0i32; 38];
    tribs[0] = 0;
    tribs[1] = 1;
    tribs[2] = 1;

    for i in 3..=n as usize {
        tribs[i] = tribs[i - 1] + tribs[i - 2] + tribs[i - 3];
    }
    tribs[n as usize]
}

#[test]
fn run() {
    assert_eq!(tribonacci(25), 1389537);
}
