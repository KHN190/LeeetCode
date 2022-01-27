// https://leetcode.com/problems/decode-string/

// it can embed [] in another []
// so we should build temp res, and do it recursively
// or using a stack: stack contains the i repeating num

pub fn decode_string(s: String) -> String {
    let mut stack: Vec<(usize, String)> = Vec::new();
    let (mut n, mut str) = (0, String::new());
    for c in s.chars() {
        match c {
            '[' => {
                stack.push((n, str.clone()));
                n = 0;
                str.clear();
            }
            ']' => {
                if let Some(last) = stack.pop() {
                    str = last.1 + str.repeat(last.0).as_str();
                }
            }
            '0'..='9' => n = n * 10 + (c as u8 - b'0') as usize,
            c => str.push(c),
        }
    }
    str
}

#[test]
fn run() {
    let s: String = "3[a]2[bc]".into();
    assert_eq!(&decode_string(s), &"aaabcbc");

    let s: String = "3[a2[c]]".into();
    assert_eq!(&decode_string(s), &"accaccacc");
}
