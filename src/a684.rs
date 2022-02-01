// https://leetcode.com/problems/redundant-connection/

// return the last edge can be removed
// detect cycle

use std::collections::HashSet;

pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
    vec![]
}

fn dfs(curr: i32, edges: &Vec<Vec<i32>>, visited: &mut HashSet<i32>, dups: &mut Vec<i32>) {
    // if edge ends with visited, it can be removed
    if visited.contains(&curr) {
        return;
    }
    visited.insert(curr);
}
