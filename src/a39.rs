// https://leetcode.com/problems/combination-sum/

// candidates are unique nums
// the result is ensured less than 150 nums in it
//

// any num <= target is a possible candidate
// then the question is how do I ensure the uniqueness of result?
// hash table (hashset)? prefix tree?

// other people's solution is DFS

pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = vec![];
    let mut nums: Vec<i32> = candidates;
    nums.sort();

    // search all combinations in dfs manner
    let remain = target;
    dfs(&nums, &mut vec![], 0, &mut res, remain);

    res
}

// cur_i: current index we are at nums
// stack: current nums we are trying to sum up to target
fn dfs(nums: &Vec<i32>, stack: &mut Vec<i32>, cur_i: usize, res: &mut Vec<Vec<i32>>, remain: i32) {
    // push to result if adds up to target
    if remain == 0 {
        res.push(stack.clone());
        return;
    }
    // else, add next from nums to cur
    let mut i = cur_i;
    while i < nums.len() && nums[i] <= remain {
        stack.push(nums[i]);
        dfs(nums, stack, i, res, remain - nums[i]);
        stack.pop();
        i += 1;
    }
}

#[test]
fn run() {
    let nums = vec![2, 3, 6, 7];
    assert_eq!(combination_sum(nums, 7), vec![vec![2, 2, 3], vec![7]]);
}
