// https://leetcode.com/problems/longest-substring-with-at-least-k-repeating-characters/

pub fn longest_substring(s: String, k: i32) -> i32 {
    let s = s.as_bytes();
    let mut answer = 0;
    for i in 1..=26 {
        let mut dict = [0; 26];
        // lhs + rhs pointer
        let (mut l, mut r) = (0, 0);
        let mut uniq = 0;
        let mut ok = 0;
        while r < s.len() {
            if uniq <= i {
                // char idx in the dict
                let idx = (s[r] - b'a') as usize;
                // we meet a new char
                if dict[idx] == 0 {
                    uniq += 1;
                }
                // increase the times of the char
                dict[idx] += 1;
                // we meet a new char at least k times
                if dict[idx] == k {
                    ok += 1;
                }
                r += 1;
            } else {
                let idx = (s[l] - b'a') as usize;
                if dict[idx] == k {
                    ok -= 1;
                }
                dict[idx] -= 1;
                if dict[idx] == 0 {
                    uniq -= 1;
                }
                l += 1;
            }
            if uniq == i && uniq == ok {
                answer = answer.max(r - l)
            }
        }
    }
    answer as i32
}
