// https://leetcode.com/problems/subsets/

pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = vec![vec![]];
    if nums.len() == 1 {
        res.push(vec![nums[0]]);
        return res;
    }
    for i in 0..nums.len() {
        let n1 = nums[i];
        let mut tmp: Vec<Vec<i32>> = vec![vec![n1]];
        for j in (i + 1)..nums.len() {
            let n2 = nums[j];
            for v in &tmp.clone() {
                let mut v2 = v.clone();
                v2.push(n2);
                tmp.push(v2);
            }
        }
        res.append(&mut tmp);
    }
    res
}

#[test]
fn run() {
    let mut res = subsets(vec![1, 2, 3]);
    let ans = vec![
        vec![],
        vec![1],
        vec![1, 2],
        vec![1, 2, 3],
        vec![1, 3],
        vec![2],
        vec![2, 3],
        vec![3],
    ];
    res.sort();

    assert_eq!(res, ans);
}
