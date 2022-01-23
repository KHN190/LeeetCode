// https://leetcode.com/problems/min-cost-climbing-stairs/

// You are given an integer array cost where cost[i] is the cost of ith step on a staircase.
// Once you pay the cost, you can either climb one or two steps.
// You can either start from the step with index 0, or the step with index 1.

// find min cost
// 1. what is the start?
// 2. transition is easy to find:
//      res[i] = (res[i - 1] + cost[i - 1]).min(res[i - 2] + cost[i - 2])
// start[0] = 0
// start[1] = 0

pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let mut res = [0; 1001];
    for i in 2..=cost.len() {
        res[i] = (res[i - 1] + cost[i - 1]).min(res[i - 2] + cost[i - 2]);
    }
    res[cost.len()]
}

#[test]
fn run() {
    let cost = vec![10, 15, 20];
    assert_eq!(min_cost_climbing_stairs(cost), 15);
}
