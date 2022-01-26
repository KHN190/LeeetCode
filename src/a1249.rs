// https://leetcode.com/problems/minimum-remove-to-make-valid-parentheses/

// follow up: LC921

pub fn min_remove_to_make_valid(s: String) -> String {
    let mut stack: Vec<usize> = vec![];
    let mut skip_idx: Vec<usize> = vec![];
    for (i, c) in s.chars().enumerate() {
        match c {
            '(' => {
                stack.push(i);
            }
            ')' => {
                if !stack.is_empty() {
                    stack.pop();
                } else {
                    skip_idx.push(i);
                }
            }
            _ => continue,
        }
    }
    while !stack.is_empty() {
        skip_idx.push(stack.pop().unwrap());
    }
    // println!("del parenth: {:?}", skip_idx.len());

    s.char_indices()
        .filter_map(|(i, c)| if skip_idx.contains(&i) { None } else { Some(c) })
        .collect()
}

#[test]
fn run() {
    let s = min_remove_to_make_valid("lee(t(c)o)de)".into());
    assert_eq!(&s, &"lee(t(c)o)de");

    let s = min_remove_to_make_valid("a)b(c)d".into());
    assert_eq!(&s, &"ab(c)d");
}
