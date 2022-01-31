// https://leetcode.com/problems/number-of-provinces/

// detect all the connected regions
// dfs

use std::collections::HashSet;

// is_connected [n x n], 1: connected
pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
    let mut n = 0i32;
    let mut visited = HashSet::<i32>::new();
    for i in 0..is_connected[0].len() {
        if !visited.contains(&(i as i32)) {
            n += 1;
        }
        dfs(i, &is_connected, &mut visited);
    }
    n
}

fn dfs(curr: usize, edges: &Vec<Vec<i32>>, visited: &mut HashSet<i32>) {
    if visited.contains(&(curr as i32)) {
        return;
    }
    visited.insert(curr as i32);
    // visit all neighbours
    for i in 0..edges[0].len() {
        if edges[curr][i] == 1 {
            dfs(i, edges, visited);
        }
    }
}

#[test]
fn run() {
    let is_connected: Vec<Vec<i32>> = vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]];
    assert_eq!(find_circle_num(is_connected), 2);
}
