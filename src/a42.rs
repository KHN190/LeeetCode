// https://leetcode.com/problems/trapping-rain-water/

// track i & height to calc water volume
// @todo it's much more intuitive to use pointers
//       mono stack is the same as pointers.

pub fn trap(height: Vec<i32>) -> i32 {
    let mut stack = Vec::<(usize, i32)>::new();
    let mut res: i32 = 0;

    stack.push((0, -1));

    for (j, rhs) in height.iter().enumerate() {
        // when we have more than one value,
        // add to result if possible.
        while stack.len() > 1 {
            let (_, cur) = stack.last().unwrap();
            // we are done when all lhs <= rhs
            // and all volume are added to res
            if cur > rhs {
                break;
            }
            let (_, cur) = stack.pop().unwrap();
            let (i, lhs) = stack.last().unwrap();
            // add to res, otherwise
            let cur_h = rhs.min(lhs) - cur;
            if cur_h > 0 {
                res += (j - i - 1) as i32 * cur_h;
            }
        }
        stack.push((j, *rhs));
    }
    res
}

#[test]
fn run() {
    assert_eq!(trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    assert_eq!(trap(vec![4, 2, 0, 3, 2, 5]), 9);
}
