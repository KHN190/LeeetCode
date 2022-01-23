// https://leetcode.com/problems/house-robber-ii/

// same as #198. but houses are in circle.
// for 0 and n-1. (choose between these two)
// otherwise, it's the *same* with #198.

fn simple_rob(nums: &[i32]) -> i32 {
    let mut rob: i32 = 0; // max if rob cur
    let mut not_rob: i32 = 0; // max if not rob cur
    for n in nums {
        let cur = n + not_rob;
        not_rob = not_rob.max(rob);
        rob = cur;
    }
    not_rob.max(rob)
}

pub fn rob(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n <= 2 {
        return *nums.iter().max().unwrap();
    }
    simple_rob(&nums[1..]).max(simple_rob(&nums[..n - 1]))
}

#[test]
fn run() {
    assert_eq!(rob(vec![2, 3, 2]), 3);
    assert_eq!(rob(vec![1, 2, 3, 1]), 4);
    assert_eq!(rob(vec![1, 2, 3]), 3);
    assert_eq!(rob(vec![1, 1, 1, 2]), 3);
}
