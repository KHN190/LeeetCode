// https://leetcode.com/problems/remove-k-digits/
// https://leetcode.com/problems/remove-k-digits/discuss/88668/Short-Python-one-O(n)-and-one-RegEx

pub fn remove_kdigits(num: String, k: i32) -> String {
    let mut k = k as usize;
    let mut stack: Vec<char> = vec![];
    for c in num.chars() {
        // stack always contains the smallest result on leftmost,
        // with at most k deletion.
        // this problem is greedy but I can't prove it
        while k > 0 && stack.len() > 0 && stack.last().unwrap() > &c {
            stack.pop();
            k -= 1;
        }
        // append curr char but skip leading zeros
        if stack.is_empty() && c == '0' {
            continue;
        }
        stack.push(c);
    }
    // build result
    if stack.is_empty() || stack.len() <= k {
        return "0".into();
    }
    stack[0..stack.len() - k].iter().collect()
}

#[test]
fn run() {
    assert_eq!(&remove_kdigits("1432219".into(), 3), "1219");
    assert_eq!(&remove_kdigits("10200".into(), 1), "200");
    assert_eq!(&remove_kdigits("9".into(), 1), "0");
    assert_eq!(&remove_kdigits("10001".into(), 4), "0");
}
