// https://leetcode.com/problems/regular-expression-matching/

// aa, a* -> true
// ab, .* -> true
// aab, c*a*b -> true

// cur char in p, with length (. or *)
// cur char in s
// cur pos in p
// cur pos in s

// recursive is still necessary because you need
// backtracking (a*a, .*aa, etc.)

pub fn is_match(s: String, p: String) -> bool {
    is_match_str(&s, &p)
}

fn is_match_str(s: &str, p: &str) -> bool {
    let slen = s.len();
    let plen = p.len();
    if plen == 0 && slen == 0 {
        return true;
    }

    let cur_s = s.chars().nth(0);
    let cur_p = p.chars().nth(0);
    // match single char?
    let m = { cur_s == cur_p || cur_p == Some('.') };
    // if pattern comes next with *
    if p.chars().nth(1) == Some('*') {
        // a* can match nothing
        // or advance single char, pattern doesn't change
        return (plen >= 2 && is_match_str(s, &p[2..]))
            || (m && slen >= 1 && is_match_str(&s[1..], p));
    }
    // otherwise, both advance single char
    m && (slen >= 1 && plen >= 1 && is_match_str(&s[1..], &p[1..]))
}

#[test]
fn run() {
    assert_eq!(is_match("aa".into(), "a*".into()), true);
    assert_eq!(is_match("ab".into(), ".*".into()), true);
    assert_eq!(is_match("aab".into(), "c*a*b".into()), true);
    assert_eq!(is_match("aab".into(), "c*a*bb".into()), false);
    assert_eq!(is_match("mississippi".into(), "mis*is*ip*.".into()), true);
    assert_eq!(is_match("aaa".into(), "a*a".into()), true);
    assert_eq!(is_match("ab".into(), ".*c".into()), false);
    assert_eq!(is_match("a".into(), ".*..a*".into()), false);
}
