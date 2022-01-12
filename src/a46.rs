// https://leetcode.com/problems/permutations/

// return all permutations
// nums are unique

pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = vec![];
    backtrack(&nums, &vec![], &mut res);
    res
}

fn backtrack(nums: &[i32], sub: &[i32], res: &mut Vec<Vec<i32>>) {
    if nums.len() == 0 {
        res.push(sub.to_vec());
        return;
    }
    for (i, v) in nums.iter().enumerate() {
        let (mut nums_c, mut sub_c) = (nums.to_vec(), sub.to_vec());
        nums_c.remove(i);
        sub_c.push(*v);
        backtrack(&nums_c, &sub_c, res);
    }
}

#[test]
fn run() {
    let nums = vec![1, 2];
    assert_eq!(permute(nums), vec![[1, 2], [2, 1]]);
}
