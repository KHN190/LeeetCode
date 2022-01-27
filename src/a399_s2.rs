use std::collections::{HashMap, HashSet, VecDeque};

pub fn calc_equation(
    equations: Vec<Vec<String>>,
    values: Vec<f64>,
    queries: Vec<Vec<String>>,
) -> Vec<f64> {
    let mut graph: HashMap<&String, Vec<(f64, &String)>> = HashMap::new();
    for i in 0..equations.len() {
        (*graph.entry(&equations[i][0]).or_insert(vec![])).push((values[i], &equations[i][1]));
        (*graph.entry(&equations[i][1]).or_insert(vec![]))
            .push((1.0 / values[i], &equations[i][0]));
    }
    queries
        .into_iter()
        .map(|v| {
            let mut seen = HashSet::new();
            let mut q: VecDeque<(f64, &String)> = VecDeque::new();
            if let Some(edges) = graph.get(&v[0]) {
                for edge in edges {
                    q.push_back(edge.clone());
                }
            }
            while let Some((val, s)) = q.pop_front() {
                seen.insert(s.to_owned());
                if s == &v[1] {
                    return val;
                } else {
                    for edge in graph.get(&s).unwrap_or(&vec![]) {
                        if !seen.contains(edge.1) {
                            q.push_back((val * edge.0, edge.1));
                        }
                    }
                }
            }
            -1.0
        })
        .collect()
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
