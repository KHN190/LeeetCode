// determine if there's a missing number

pub fn missing_number(nums: Vec<i32>) -> bool {
    let mut n: i32 = 0;
    let mut sum: i32 = 0;
    for i in nums.iter() {
        if i > &n { n = *i; }
        sum += i;
    }
    (n + 1) * n / 2 != sum
}

#[test]
fn run() {
    assert_eq!(missing_number(vec![3, 0, 1]), true);
    assert_eq!(missing_number(vec![3, 0, 1, 2]), false);
}
