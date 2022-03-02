// https://leetcode.com/problems/nested-list-weight-sum/

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

pub fn depth_sum(nested_list: Vec<NestedInteger>) -> i32 {
    recursive(nested_list, 1)
}

fn recursive(nested_list: Vec<NestedInteger>, level: i32) -> i32 {
    let mut res = 0;
    for e in nested_list {
        match e {
            NestedInteger::Int(val) => res += level * val,
            NestedInteger::List(l) => res += recursive(l, level + 1),
        }
    }
    res
}

#[test]
fn run() {}
