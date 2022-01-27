// https://leetcode.com/discuss/interview-question/380650/

// extra excercise
// 1d candy crash

// eliminate 3 consequent identical numbers

pub fn candy_crash(nums: Vec<i32>) -> Vec<i32> {
    let mut stack: Vec<Vec<i32>> = vec![];
    for n in &nums {
        op_stack(&mut stack, *n);
    }
    if stack.len() > 0 && stack[stack.len() - 1][1] >= 3 {
        stack.pop();
    }
    // concat stack
    let mut res = vec![];
    for e in stack {
        for _ in 0..e[1] {
            res.push(e[0]);
        }
    }
    res
}

fn op_stack(stack: &mut Vec<Vec<i32>>, n: i32) {
    if stack.is_empty() {
        stack.push(vec![n, 1]);
        return;
    }

    let mut tail = stack.pop().unwrap();
    // increase tail if val is repeating
    if tail[0] == n {
        tail[1] += 1;
        stack.push(tail);
        return;
    }
    // crush when tail not repeating
    if tail[1] < 3 {
        stack.push(tail);
        stack.push(vec![n, 1]);
        return;
    }
    op_stack(stack, n);
}

// recursively or using a stack

#[test]
fn run() {
    let nums = vec![1, 2, 2, 2, 2, 1, 1, 3];
    assert_eq!(candy_crash(nums), vec![3]);

    let nums = vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 5, 4, 3, 2, 1];
    assert_eq!(candy_crash(nums), vec![]);
}
