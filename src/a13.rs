// https://leetcode.com/problems/roman-to-integer/

// 1 <= s.length <= 15
// s contains only the characters ('I', 'V', 'X', 'L', 'C', 'D', 'M').
// It is guaranteed that s is a valid roman numeral in the range [1, 3999].

// 這實際上是一道 parser 相關的題。

#[allow(dead_code)]
pub fn roman_to_int(s: String) -> i32 {
    use std::collections::HashMap;
    // static dict from Roman to int
    let roman_dict: HashMap<char, i32> = [
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]
    .iter()
    .cloned()
    .collect();

    // if next char is greater than this char, we deduct
    // otherwise, just add from left to right
    let mut num: i32 = 0;
    let mut text: &str = &s;

    while text.len() > 0 {
        // peek this char and next char
        let next_char = &text.chars().nth(1).unwrap_or('\0');
        let curr_char = &text.chars().nth(0).unwrap();
        // convert char -> int
        let curr_val = roman_dict[&curr_char];
        let next_val = if next_char == &'\0' {
            0
        } else {
            roman_dict[next_char]
        };

        if next_val > curr_val {
            num += next_val - curr_val;
            // consume current text twice
            text = next(next(text));
        } else {
            num += curr_val;
            // consume current text once
            text = next(text);
        }
    }

    num
}

fn next(s: &str) -> &str {
    &s[1..s.len()]
}

#[test]
fn run() {
    assert_eq!(roman_to_int("IX".into()), 9);
    assert_eq!(roman_to_int("IV".into()), 4);
    assert_eq!(roman_to_int("LVIII".into()), 58);
    assert_eq!(roman_to_int("MCMXCIV".into()), 1994);
}
