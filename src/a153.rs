// https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/

// nums is sorted but shifted. find the min.
// must be O(log(N)).

pub fn find_min(nums: Vec<i32>) -> i32 {
    let mut lhs = 0;
    let mut rhs = nums.len() - 1;

    // stop exactly at [.., mid, rhs, ..]
    //     follow up: what if nums are not unique? (mid >= rhs to skip)
    while lhs < rhs {
        let mid = lhs + (rhs - lhs) / 2;
        // not in ascending order, so num is at the right
        if nums[mid] > nums[rhs] {
            lhs = mid + 1;
        } else {
            // num is at the left
            rhs = mid;
        }
    }
    nums[lhs]
}

#[test]
fn run() {
    let v = vec![4, 5, 6, 7, 0, 1, 2];
    assert_eq!(find_min(v), 0);
}
