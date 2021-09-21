// https://leetcode.com/problems/container-with-most-water/

// Find two bars, that between them it contains the most water.

// n == height.length
// 2 <= n <= 10^5
// 0 <= height[i] <= 10^4

//
// Thoughts:
//  Not to find top 2.
//  V(water) = abs(i - j) * min(n_i, n_j)
//
//  Worst case: N^2 by brute force.
//  But we can cache (?).
//  Only the final volume is asked. (can we make use of it?)
//
//  Start from both ends, move the shorter end, compare volume,
//  keep the max. (scan from both sides -> ensures better res).

pub fn max_area(height: Vec<i32>) -> i32 {
    let mut i: usize = 0;
    let mut j: usize = height.len() - 1;
    let mut max: i32 = 0;

    loop {
        if i >= j {
            break;
        }

        let a1 = height[i];
        let a2 = height[j];
        let v: i32;

        if a1 < a2 {
            v = (j - i) as i32 * a1;
            i += 1;
        } else {
            v = (j - i) as i32 * a2;
            j -= 1;
        }
        if v > max {
            max = v;
        }
    }
    max
}

#[test]
fn run() {
    let v = vec![4, 3, 2, 1, 4];
    assert_eq!(max_area(v), 16);

    let v = vec![1, 1];
    assert_eq!(max_area(v), 1);

    let v = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    assert_eq!(max_area(v), 49);

    let v = vec![1, 2, 1];
    assert_eq!(max_area(v), 2);
}
