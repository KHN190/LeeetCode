// https://leetcode.com/problems/longest-valid-parentheses/

// Given a string containing just the characters '(' and ')',
// find the length of the longest valid (well-formed) parentheses substring.

// 0 <= s.length <= 3 * 10^4
// s[i] is '(', or ')'.

// res[i] is number of valid parentheses matches
// res[i] = res[i-1] + 2 + res[i - (res[i-1] + 2)] if s[i] = ')' and '(' count > 0

pub fn longest_valid_parentheses(s: String) -> i32 {
    // current max match at i
    let mut res: Vec<usize> = vec![0; s.len()];
    let mut open: i32 = 0;
    let mut cur_par: usize = 0;
    let mut max: i32 = 0;

    for (i, c) in s.chars().enumerate() {
        if c == '(' {
            open += 1;
            cur_par = 0;
        }
        if c == ')' && open > 0 {
            open -= 1;
            cur_par += 1;
            // Match found
            res[i] = 2 + res[i - 1];
            // Match from prev
            // let cur = res[i];
            // if i > cur {
            //     res[i] += res[i - cur];
            // }
            if i > cur_par * 2 {
                res[i] += res[i - cur_par * 2];
            }
        }
        if res[i] as i32 > max {
            max = res[i] as i32;
        }
    }
    max
}

#[test]
fn run() {
    assert_eq!(longest_valid_parentheses("(()".into()), 2);
    assert_eq!(longest_valid_parentheses(")()())".into()), 4);
    assert_eq!(longest_valid_parentheses("".into()), 0);
    assert_eq!(longest_valid_parentheses("()(()".into()), 2);
    assert_eq!(longest_valid_parentheses("()(())".into()), 6);
}
