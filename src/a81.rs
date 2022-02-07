// https://leetcode.com/problems/search-in-rotated-sorted-array-ii/

// nums is ascending, but has duplicates.
// and it is shifted.

pub fn search(nums: Vec<i32>, target: i32) -> bool {
    if nums.len() == 0 {
        return false;
    }
    let (mut l, mut h) = (0, nums.len() - 1);
    while l < h {
        let mid = l + (h - l) / 2;
        if nums[mid] == target {
            return true;
        }
        // [.., n11, mid, n0, n1, ..]
        if nums[mid] > nums[h] {
            if nums[mid] > target && nums[l] <= target {
                h = mid;
            } else {
                l = mid + 1;
            }
            continue;
        }
        // [.., n9, mid, n11, n12, ..]
        if nums[mid] < nums[h] {
            if nums[mid] < target && nums[h] >= target {
                l = mid + 1;
            } else {
                h = mid;
            }
            continue;
        }
        h -= 1;
    }
    nums[l] == target
}

#[test]
fn run() {
    let v = vec![2, 5, 6, 0, 0, 1, 2];
    assert!(search(v, 0));

    let v = vec![2, 5, 6, 0, 0, 1, 2];
    assert!(search(v, 2));

    let v = vec![2, 5, 6, 0, 0, 1, 2];
    assert!(!search(v, 3));

    let v = vec![1, 0, 1, 1, 1];
    assert!(search(v, 0));

    let v = vec![2, 2, 2, 3, 2, 2, 2];
    assert!(search(v, 3));
}
