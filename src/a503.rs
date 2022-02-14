// https://leetcode.com/problems/next-greater-element-ii/

// same as #496 but circular array

pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
    let mut stack = vec![];
    let mut res = vec![-1; nums.len()];
    // build a dict of all greater numbers
    for k in 0..(nums.len() * 2) {
        let i = k % nums.len();
        let n = nums[i];
        while !stack.is_empty() && nums[*stack.last().unwrap()] < n {
            res[stack.pop().unwrap()] = n;
        }
        stack.push(i);
    }
    res
}

#[test]
fn run() {
    let n = vec![100, 1, 11, 1, 120, 111, 123, 1, -1, -100];
    let res = next_greater_elements(n);
    assert_eq!(res, vec![120, 11, 120, 120, 123, 123, -1, 100, 100, 100]);
}
