// https://leetcode.com/problems/evaluate-division/
// https://leetcode.com/discuss/interview-question/483660/Google-or-Phone-or-Currency-Conversion

// directed graph

use std::collections::{HashMap, HashSet};

type Graph = HashMap<String, HashMap<String, f64>>;

pub fn calc_equation(
    equations: Vec<Vec<String>>,
    values: Vec<f64>,
    queries: Vec<Vec<String>>,
) -> Vec<f64> {
    let graph = build_graph(equations, values);

    queries
        .into_iter()
        .map(|q| dfs(&graph, q, &mut HashSet::new(), None).unwrap_or(-1.0))
        .collect()
}

fn build_graph(equations: Vec<Vec<String>>, values: Vec<f64>) -> Graph {
    let mut graph = Graph::new();
    for i in 0..equations.len() {
        let val = values[i];
        let eq = &equations[i];

        graph
            .entry(eq[0].clone())
            .or_default()
            .insert(eq[1].clone(), val);
        graph
            .entry(eq[1].clone())
            .or_default()
            .insert(eq[0].clone(), 1.0 / val);
    }
    graph
}

fn dfs(
    graph: &Graph,
    q: Vec<String>,
    visited: &mut HashSet<String>,
    curr: Option<f64>,
) -> Option<f64> {
    // handle same node
    if q[0] == q[1] {
        return Some(1.0);
    }
    // node not exists
    if !graph.contains_key(&q[0]) {
        return None;
    }
    // if we have a direct connection
    let entry = &graph[&q[0]];
    if let Some(edge) = entry.get(&q[1]) {
        return Some(*edge * curr.unwrap_or(1.0));
    }
    // we don't have a direct connection, dfs
    for node in entry.keys() {
        if !visited.contains(node) {
            if let Some(edge) = entry.get(node) {
                let res = dfs(
                    graph,
                    vec![node.clone(), q[1].clone()],
                    visited,
                    Some(*edge * curr.unwrap_or(1.0)),
                );
                if res.is_some() {
                    return res;
                }
            }
        }
    }
    None
}

#[test]
fn run() {
    let equations: Vec<Vec<String>> = vec![
        vec!["a".into(), "b".into()],
        vec!["b".into(), "c".into()],
        vec!["bc".into(), "cd".into()],
    ];
    let values = vec![1.5, 2.5, 5.0];
    let queries: Vec<Vec<String>> = vec![
        vec!["a".into(), "c".into()],
        vec!["c".into(), "b".into()],
        vec!["bc".into(), "cd".into()],
        vec!["cd".into(), "bc".into()],
    ];

    let res = calc_equation(equations, values, queries);
    assert_eq!(res, vec![3.75, 0.4, 5.0, 0.2]);
}
