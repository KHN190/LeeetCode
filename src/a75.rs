// https://leetcode.com/problems/sort-colors/

// Sort array without using library's sort
// Contains only 0, 1, 2

pub fn sort_colors(nums: &mut Vec<i32>) {
    // counter for 0, 1; remains are 2
    let (mut c0, mut c1) = (0, 0);
    for n in nums.iter() {
        match n {
            0 => {
                c0 += 1;
            }
            1 => {
                c1 += 1;
            }
            _ => {}
        }
    }
    for i in 0..nums.len() {
        nums[i] = if i < c0 {
            0
        } else if i < c0 + c1 {
            1
        } else {
            2
        };
    }
}

#[test]
fn run() {
    let mut nums = vec![0, 2, 2, 1, 0];
    sort_colors(&mut nums);

    assert_eq!(nums, vec![0, 0, 1, 2, 2]);
}
