// https://leetcode.com/problems/two-city-scheduling/description/

// we can find savings, directing all to A by [c1 - c2, ..] and maximize it.
//
// 1. so we sort the costs by that strategy
// 2. optimal: quick_select

pub fn two_city_sched_cost(mut costs: Vec<Vec<i32>>) -> i32 {
    costs.sort_by(|a, b| (a[0] - a[1]).partial_cmp(&(b[0] - b[1])).unwrap());

    let n = costs.len() / 2;
    let mut res = 0;
    for i in 0..n {
        res += costs[i][0] + costs[i + n][1];
    }
    res
}

#[test]
fn run() {
    let costs: Vec<Vec<i32>> = vec![vec![10, 20], vec![30, 200], vec![400, 50], vec![30, 20]];
    assert_eq!(two_city_sched_cost(costs), 110);
}
