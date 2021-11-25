// https://leetcode.com/problems/climbing-stairs/

// n stairs to reach the end
// you can step 1 or 2
// how many unique paths?

pub fn climb_stairs(n: i32) -> i32 {
    // at any i, res[i] = res[i - 1] + res[i - 2]
    if n <= 0 {
        return 0;
    }
    let n: usize = n as usize;
    let mut res: Vec<i32> = vec![1, 2];
    for i in 2..n {
        res.push(res[i - 1] + res[i - 2]);
    }
    res[n - 1]
}

#[test]
fn run() {
    assert_eq!(climb_stairs(1), 1);
    assert_eq!(climb_stairs(2), 2);
    assert_eq!(climb_stairs(3), 3);
}
