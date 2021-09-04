// https://leetcode.com/problems/sliding-window-maximum/

// You are given an array of integers nums, there is a sliding window of
// size k which is moving from the very left of the array to the very right.
// You can only see the k numbers in the window.
// Each time the sliding window moves right by one position.
//
// Return the max sliding window.
//
// 1 <= nums.length <= 10^5
// -10^4 <= nums[i] <= 10^4
// 1 <= k <= nums.length

pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    use std::collections::VecDeque;
    // for each window, push all elements to ~~stack~~ (queue)
    // and peek the max val from queue, save to result

    // Mono stack doesn't keep max number
    let n = nums.len();
    let k = k as usize;
    let mut res: Vec<i32> = Vec::new();
    let mut q = VecDeque::new();
    for i in 0..n {
        // remove old vals
        while let Some(&id) = q.front() {
            if id + k <= i {
                q.pop_front();
            } else {
                break;
            }
        }
        // remove larger vals, then push new val
        while let Some(&id) = q.back() {
            if nums[id] <= nums[i] {
                q.pop_back();
            } else {
                break;
            }
        }
        q.push_back(i);
        // max val at front
        if i >= k - 1 {
            if let Some(&id) = q.front() {
                res.push(nums[id]);
            }
        }
    }
    res
}

#[test]
fn run() {
    let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];

    // 1
    // 3
    // 3, -1
    // 3, -1, -3
    // 5
    // 5, 3
    // 6
    // 7
    let res = max_sliding_window(nums.clone(), 3);
    assert_eq!(res, vec![3, 3, 5, 5, 6, 7]);

    // for mono stack
    // 1
    // 1 3
    // -1
    // -3
    // -3, 5
    // -3, 3
    // -3, 3, 6
    // -3, 3, 6, 7
}
