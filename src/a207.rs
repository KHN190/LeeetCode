// https://leetcode.com/problems/course-schedule/

// detect cycle in a graph. if there's a cycle,
// then impossible to finish all course.
// cycle means starts from a point, you can visit it again.

use std::collections::HashMap;
use std::collections::HashSet;

// 0: being visited, 1: visited, -1: not visited
pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let (graph, nodes) = create_graph(prerequisites);
    let visited = vec![-1; nodes.len()];

    for node in nodes {
        //
    }

    false
}

// return graph and all nodes
fn create_graph(edges: Vec<Vec<i32>>) -> (HashMap<i32, Vec<i32>>, Vec<i32>) {
    let mut graph = HashMap::<i32, Vec<i32>>::new();
    let mut nodes = HashSet::<i32>::new();
    for edge in edges {
        let (k, v) = (edge[1], edge[0]);
        graph.entry(k).or_insert(vec![]).push(v);
        nodes.insert(k);
        nodes.insert(v);
    }
    (graph, nodes.into_iter().collect())
}
