// https://leetcode.com/problems/next-permutation/

// Implement next permutation, which rearranges numbers into
// the lexicographically next greater permutation of numbers.
// otherwise,
// return the lowest possible number.

// it means [1,1,2,3] -> [1,1,3,2]
//          [1,2,3,1] -> [1,2,1,3] -> [1,3,1,2]
//          [3,2,3,1] -> [3,2,1,3] -> [3,3,1,2]
// 3 is moved before 2, and

// ~~~
// it means sort the rhs list after the 2nd largest num,
// keep all orders on the lhs.
// then swap the largest and the 2nd largest.
// ~~~ (wrong when [1,2,4,3])

// what if it has 0?

// 3 is the greatest number, and it's moved before the 2nd largest number (2)
// then ensure after 3 they are all ascending

// only if they all descending, there's no next greater number
// then return reversed order

// nums.len() >= 1
// constant memory + in-place: only index op

// See wiki:
//   https://en.wikipedia.org/wiki/Permutation#Generation_in_lexicographic_order

pub fn next_permutation(nums: &mut Vec<i32>) {
    let s = nums.as_mut_slice();
    let mut i = s.len() - 1;
    while i >= 1 && s[i - 1] >= s[i] {
        i -= 1;
    }
    if i >= 1 {
        let mut j = s.len() - 1;
        while s[j] <= s[i - 1] {
            j -= 1;
        }
        s.swap(i - 1, j);
    }
    s[i..].reverse();
}

#[test]
fn run() {
    let mut nums = vec![1, 2, 3];
    next_permutation(&mut nums);

    assert_eq!(nums, vec![1, 3, 2]);

    let mut nums = vec![3, 2, 1];
    next_permutation(&mut nums);

    assert_eq!(nums, vec![1, 2, 3]);
}
