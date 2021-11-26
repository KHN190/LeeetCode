// https://leetcode.com/problems/house-robber/

// f(0) = nums[0]
// f(1) = max(num[0], num[1])
// f(k) = max( f(k-2) + nums[k], f(k-1) )

pub fn rob(nums: Vec<i32>) -> i32 {
    let mut rob: i32 = 0; // max if rob cur
    let mut not_rob: i32 = 0; // max if not rob cur
    for n in nums {
        let cur = n + not_rob;
        not_rob = not_rob.max(rob);
        rob = cur;
    }
    not_rob.max(rob)
}

#[test]
fn run() {
    assert_eq!(rob(vec![1, 2, 3, 1]), 4);
    assert_eq!(rob(vec![2, 7, 9, 3, 1]), 12);
}
