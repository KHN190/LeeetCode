// https://leetcode.com/problems/generate-parentheses/

// Given n pairs of parentheses, write a function to
// generate all combinations of well-formed parentheses.

// 1 <= n <= 8

// Thoughts:
//  paren = (*)* -> ((*)*)* -> (((*)*)*)* ((()))
//                             ((*)(*)*)* (()())
//                             ((*)*)(*)* (())()
//                  (*)(*)* -> ((*)*)(*)* (())() <- dup!
//                             (*)((*)*)* ()(())
//                             (*)(*)(*)* ()()()
//  where * is a place to add new paren
//
//  use 0 = *, 1 = (, 2 = ) and queue

pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut res: Vec<String> = vec![];
    _gen(&mut res, n, n, "".into());
    res
}

// DFS solution
pub fn _gen(res: &mut Vec<String>, open: i32, close: i32, cur: String) {
    if open == 0 && close == 0 {
        res.push(cur);
        return;
    }
    if open > 0 {
        _gen(res, open - 1, close, cur.clone() + "(");
    }
    if close > open {
        _gen(res, open, close - 1, cur.clone() + ")");
    }
}

#[test]
fn run() {
    let ans = vec![String::from("(())"), String::from("()()")];
    assert_eq!(generate_parenthesis(2), ans);
}
