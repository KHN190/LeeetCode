// https://leetcode.com/problems/maximum-product-subarray/

pub fn max_product(nums: Vec<i32>) -> i32 {
    let mut res: i32 = nums[0];
    let mut max: i32 = res;
    let mut min: i32 = res;

    for i in 1..nums.len() {
        let cur = nums[i];
        if cur < 0 {
            let tmp = max;
            max = min;
            min = tmp;
        }
        max = cur.max(cur * max);
        min = cur.min(cur * min);
        res = res.max(max);
    }
    res
}

#[test]
fn run() {
    assert_eq!(max_product(vec![2, 3, -2, 4]), 6);
    assert_eq!(max_product(vec![-2, 0, -1]), 0);
    assert_eq!(max_product(vec![1, 3, -1, 4, -10]), 120);
}
