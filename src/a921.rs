// https://leetcode.com/problems/minimum-add-to-make-parentheses-valid/

// follow up: delete parenth to make balance. (LC1249)

pub fn min_add_to_make_valid(s: String) -> i32 {
    let mut not_closed = 0;
    let mut not_opened = 0;

    for c in s.chars() {
        match c {
            '(' => {
                not_closed += 1;
            }
            ')' => {
                if not_closed > 0 {
                    not_closed -= 1;
                } else {
                    not_opened += 1;
                }
            }
            _ => unreachable!(),
        }
    }
    not_closed + not_opened
}

#[test]
fn run() {
    assert_eq!(min_add_to_make_valid("())".into()), 1);
    assert_eq!(min_add_to_make_valid("(((".into()), 3);
}
