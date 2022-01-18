// https://leetcode.com/problems/wiggle-sort-ii/

// the core idea for this problem is three-way partition
// https://leetcode.com/problems/wiggle-sort-ii/discuss/77677/O(n)%2BO(1)-after-median-Virtual-Indexing

use std::cmp::Ordering;

pub fn wiggle_sort(nums: &mut Vec<i32>) {
    if nums.is_empty() {
        return;
    }
    // rust uses timsort
    // https://en.wikipedia.org/wiki/Timsort
    nums.sort();

    let n = nums.len();
    let mut k = n - 1;

    // find median
    let median = *nums.iter().nth(n / 2).unwrap();

    // three-way partition
    let (mut i, mut j) = (0, 0);

    while j <= k {
        match (nums[index(j, n)]).cmp(&median) {
            Ordering::Less => {
                nums.swap(index(j, n), index(k, n));
                k -= 1;
            }
            Ordering::Greater => {
                nums.swap(index(j, n), index(i, n));
                j += 1;
                i += 1;
            }
            Ordering::Equal => j += 1,
        }
    }
}

// re-map index
//   [0,     mid] -> [1, 3, 5, 7,..]
//   [mid+1, n-1] -> [0, 2, 4, 6,..]
fn index(i: usize, n: usize) -> usize {
    (1 + 2 * i) % (n | 1)
}

#[test]
fn run() {
    let mut nums = vec![1, 5, 1, 1, 6, 4, 3, 1];
    wiggle_sort(&mut nums);

    assert_eq!(nums, vec![1, 5, 1, 4, 1, 6, 1, 3]);
}
