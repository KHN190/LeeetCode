// https://leetcode.com/problems/valid-word-abbreviation/

pub fn valid_word_abbreviation(word: String, abbr: String) -> bool {
    let ab = abbr.as_bytes();
    let wb = word.as_bytes();

    // number of curr char repeating
    let mut num = 0;
    // end of abbr expansion
    let mut end = 0;

    for i in 0..ab.len() {
        // if abbr is alphabet
        if ab[i] >= b'a' && ab[i] <= b'z' {
            end += num + 1;
            num = 0;
            // expansion is longer than word
            // or expanded char doesn't match
            if end >= wb.len() || ab[i] != wb[end - num - 1] {
                return false;
            }
            continue;
        }
        // if abbr is digit
        if ab[i] >= b'0' && ab[i] <= b'9' {
            // increase curr alphabet's repeating
            num = num * 10 + (ab[i] - b'0') as usize;
            continue;
        }
        // ignore illegal char
    }
    end + num == wb.len()
}

#[test]
fn run() {
    assert_eq!(
        valid_word_abbreviation("substitution".into(), "sub4u4".into()),
        true
    );

    assert_eq!(!valid_word_abbreviation("apple".into(), "a2e".into()), true);
}
