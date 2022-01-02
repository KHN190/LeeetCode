// https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/

pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // find the index, move index forward and backward if
    // the adjacent numbers are identical: O(logN) + k
    match nums.binary_search(&target) {
        Ok(i) => {
            let mut lhs = i as usize;
            let mut rhs = i as usize;

            while lhs >= 1 && nums[lhs - 1] == target {
                lhs -= 1;
            }
            while rhs < nums.len() - 1 && nums[rhs + 1] == target {
                rhs += 1;
            }
            vec![lhs as i32, rhs as i32]
        }
        _ => { vec![-1, -1] }
    }
}

#[test]
fn run() {
    let nums = vec![5,7,7,8,8,10];
    assert_eq!(search_range(nums, 8), vec![3, 4]);

    let nums = vec![5,7,7,8,8,10];
    assert_eq!(search_range(nums, 6), vec![-1, -1]);

    let nums = vec![];
    assert_eq!(search_range(nums, 0), vec![-1, -1]);
}
