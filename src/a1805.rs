// https://leetcode.com/problems/number-of-different-integers-in-a-string/

// 1 <= word.length <= 1000
// word consists of digits and lowercase English letters.

// Notes:
//
//   1. 處理多使用 std 的自帶函數，如 String::split_whitespace
//      否則還需要手動處理 trailing numbers 等 corner cases
//
//   2. 處理很大的數字 -> 實際用 String 就可以了
//      結合 char.is_numeric 等，實際可以當作大數使用

#[test]
fn run_test() {
    assert_eq!(remove_leading_zeros(String::from("01")), String::from("1"));

    assert_eq!(num_different_integers(String::from("a1b01c001")), 1); // 1,1,1
    assert_eq!(num_different_integers(String::from("leet1234code234")), 2); // 1234,234
    assert_eq!(num_different_integers(String::from("a123bc34d8ef34")), 3); // 123,34,8,34
    assert_eq!(
        num_different_integers(String::from("192383183928778851682383a2089984061937879119")),
        2
    );
}

pub fn num_different_integers(word: String) -> i32 {
    use std::collections::HashSet;
    use std::iter::FromIterator;

    // find numbers
    let nums: Vec<String> = remove_alphabets(word)
        .split_whitespace()
        .map(|x| remove_leading_zeros(x.into()))
        .collect();

    // turn vec to hashset
    let unique_nums = HashSet::<String>::from_iter(nums);

    unique_nums.len() as i32
}

fn remove_alphabets(word: String) -> String {
    word.chars()
        .map(|x| match x {
            'a'..='z' => ' ',
            _ => x,
        })
        .collect()
}

fn remove_leading_zeros(word: String) -> String {
    let mut result = String::new();
    let mut num_start = false;
    for c in word.chars() {
        if !num_start && c as usize - '0' as usize != 0 {
            num_start = true;
        }
        if num_start {
            result.push(c);
        }
    }
    result
}
