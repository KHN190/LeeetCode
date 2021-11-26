// https://leetcode.com/problems/unique-binary-search-trees/

pub fn num_trees(n: i32) -> i32 {
    if n <= 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    if n == 2 {
        return 2;
    }

    let n: usize = n as usize;
    let mut res: Vec<i32> = vec![0, 1, 2];
    // for n >= 3
    // both lhs + rhs (sum up to n - 1)
    // lhs from 0 -> n - 1
    for i in 3..=n {
        let mut cur: i32 = 0;
        for lhs in 0..i {
            let rhs: usize = i - 1 - lhs;

            if lhs == 0 || rhs == 0 {
                cur += res[lhs].max(res[rhs]);
                continue;
            }
            cur += res[rhs] * res[lhs];
        }
        res.push(cur);
    }
    res[n]
}

#[test]
fn run() {
    assert_eq!(num_trees(3), 5);
    assert_eq!(num_trees(4), 14);
}
