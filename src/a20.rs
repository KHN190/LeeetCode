// https://leetcode.com/problems/valid-parentheses/

// 1 <= s.length <= 104
// s consists of parentheses only '()[]{}'

#[test]
fn run() {
    assert!(is_valid("()[]{}".into()));
    assert!(is_valid("([]{})".into()));
    assert!(is_valid("([{{}}])".into()));
}

#[allow(dead_code)]
pub fn is_valid(s: String) -> bool {
    let mut stack: Vec<char> = vec!();
    for c in s.chars() {
        if stack.is_empty() { stack.push(c); }
        else {
            match stack.last().unwrap() {
                '(' => if c == ')' { stack.pop(); } else { stack.push(c); }
                '[' => if c == ']' { stack.pop(); } else { stack.push(c); }
                '{' => if c == '}' { stack.pop(); } else { stack.push(c); }
                _ => { return false; }
            }
        }
    }
    stack.len() == 0
}
