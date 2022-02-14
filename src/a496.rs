// https://leetcode.com/problems/next-greater-element-i/

use std::collections::HashMap;

pub fn next_greater_element(mut nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut greater_num = HashMap::<i32, i32>::new();
    let mut stack: Vec<i32> = vec![];
    // build a dict of all greater numbers
    for n in nums2 {
        // stack contains all the seen numbers
        // if they have next greater number, it's immediately inserted
        while !stack.is_empty() && stack.last().unwrap() < &n {
            greater_num.insert(stack.pop().unwrap(), n);
        }
        stack.push(n);
    }
    // find the greater number for n1
    for i in 0..nums1.len() {
        nums1[i] = *greater_num.get(&nums1[i]).unwrap_or(&-1);
    }
    nums1
}

#[test]
fn run() {
    let n1 = vec![4, 1, 2];
    let n2 = vec![1, 3, 4, 2];
    let res = next_greater_element(n1, n2);
    assert_eq!(res, vec![-1, 3, -1]);
}
