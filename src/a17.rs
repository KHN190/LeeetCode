// https://leetcode.com/problems/letter-combinations-of-a-phone-number/

pub fn letter_combinations(digits: String) -> Vec<String> {
    let mut res = Vec::<String>::new();
    for c in digits.chars() {
        let letters = to_chars(&c);
        let mut tmp: Vec<String> = vec![];
        for cc in letters {
            if res.len() == 0 {
                tmp.push(cc.to_string());
                continue;
            }
            for r in res.iter() {
                tmp.push(r.to_owned() + &cc.to_string());
            }
        }
        res = tmp;
    }
    res
}

fn to_chars(digit: &char) -> Vec<char> {
    match digit {
        '2' => vec!['a', 'b', 'c'],
        '3' => vec!['d', 'e', 'f'],
        '4' => vec!['g', 'h', 'i'],
        '5' => vec!['j', 'k', 'l'],
        '6' => vec!['m', 'n', 'o'],
        '7' => vec!['p', 'q', 'r', 's'],
        '8' => vec!['t', 'u', 'v'],
        '9' => vec!['w', 'x', 'y', 'z'],
        _ => unreachable!(),
    }
}

#[test]
fn run() {
    assert_eq!(
        letter_combinations("23".into()),
        vec!["ad", "bd", "cd", "ae", "be", "ce", "af", "bf", "cf"]
    );

    assert_eq!(
        letter_combinations("22".into()),
        vec!["aa", "ba", "ca", "ab", "bb", "cb", "ac", "bc", "cc"]
    );
}
