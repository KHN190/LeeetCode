// https://leetcode.com/problems/3sum/

// find all triplets that nums[i] + nums[j] + nums[k] == 0.
// and i != j != k.

// the solution set must not contain duplicate triplets.

// Thoughts:
//  we don't need index, so we can sort;
//  use index for access as much as possible.

pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    if nums.len() < 3 { return vec![]; }

    let mut ns:Vec<i32> = nums.clone();
    let mut res = vec![];
    ns.sort();

    for i in 0..nums.len()-2 {
        // skip duplicate left value
        if i > 0 && ns[i] == ns[i - 1] { continue; }
        // break if sum must > 0
        if ns[i] > 0 { break; }

        let mut j = i + 1;
        let mut k = nums.len() - 1;
        loop {
            if j >= k {
                break;
            }
            // iterate all triplets
            let sum = ns[i] + ns[j] + ns[k];
            if sum == 0 {
                res.push(vec!(ns[i], ns[j], ns[k]));
                // no break, because we may have multiple
                // results with same left value;
                // and skip duplicates
                while j < k && ns[j] == ns[j+1] {
                    j += 1;
                }
                while j < k && ns[k] == ns[k-1] {
                    k -= 1;
                }
                j += 1;
                k -= 1;
            }
            else if sum < 0 {
                j += 1;
            } else {
                k -= 1;
            }
        }
    }
    res
}

#[test]
fn run() {
    let nums = vec![3,0,-2,-1,1,2];
    let mut res = three_sum(nums);
    res.sort();

    assert_eq!(res.len(), 3);
    assert_eq!(res[0], vec![-2, -1, 3]);
    assert_eq!(res[1], vec![-2, 0, 2]);
    assert_eq!(res[2], vec![-1, 0, 1]);

    // no duplicates
    let nums = vec![0,0,0,0];
    let res = three_sum(nums);

    assert_eq!(res.len(), 1);
}
