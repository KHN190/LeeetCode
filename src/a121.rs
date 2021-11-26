// https://leetcode.com/problems/best-time-to-buy-and-sell-stock/

// find a window, rhs - lhs is max

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut res: i32 = 0;
    let mut min: i32 = i32::MAX;
    for p in prices {
        min = min.min(p);
        res = res.max(p - min);
    }
    res
}

#[test]
fn run() {
    assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
}
