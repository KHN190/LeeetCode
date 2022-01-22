// https://leetcode.com/problems/merge-intervals/

pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = vec![];
    // sort the intervals by start
    intervals.sort_by(|a, b| a[0].partial_cmp(&b[0]).unwrap());

    for i in 0..intervals.len() {
        let n2 = &intervals[i];
        // if we already have previous interval
        if !res.is_empty() {
            // get the last interval
            let mut n1: Vec<i32> = res[res.len() - 1].clone();
            // should it merge?
            if n1[1] >= n2[0] {
                // merge
                n1[1] = n1[1].max(n2[1]);
                res.pop();
                res.push(n1);
            } else {
                // just push the in-coming interval
                res.push(n2.clone());
            }
        } else {
            // if we have not pushed anything yet
            res.push(n2.clone());
        }
    }
    res
}

#[test]
fn run() {
    let intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
    let res = merge(intervals);
    assert_eq!(res, vec![vec![1, 6], vec![8, 10], vec![15, 18]]);

    let intervals = vec![vec![1, 4], vec![4, 5], vec![6, 10], vec![7, 9]];
    let res = merge(intervals);
    assert_eq!(res, vec![vec![1, 5], vec![6, 10]]);
}
