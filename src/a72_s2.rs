// Easier understanding

pub fn min_distance(word1: String, word2: String) -> i32 {
    let len1 = word1.len();
    let len2 = word2.len();
    // you can only insert all chars
    if len1 == 0 || len2 == 0 {
        return (len1 | len2) as i32;
    };

    let mut a = vec![vec![0i32; len2 + 1]; len1 + 1];
    for i in 0..=len1 {
        for j in 0..=len2 {
            a[i][j] = if i == 0 || j == 0 {
                // one string is empty (len = 0),
                // then insert all other word chars
                (j | i) as i32
            } else if word1.as_bytes()[i - 1] == word2.as_bytes()[j - 1] {
                // 2 chars are same
                // both words advance 1 char
                a[i - 1][j - 1]
            } else {
                // insert  = 1 + min_dist(w1, w2[1:])
                // delete  = 1 + min_dist(w1[1:], w2)
                // replace = 1 + min_dist(w1[1:], w2[1:])
                1 + a[i - 1][j - 1].min(a[i - 1][j].min(a[i][j - 1]))
            }
        }
    }
    return a[len1][len2];
}

#[test]
fn run() {
    let w1 = "abcd";
    let w2 = "abcc";

    assert_eq!(min_distance(w1.into(), w2.into()), 1);

    let w2 = "abcdee";
    assert_eq!(min_distance(w1.into(), w2.into()), 2);

    let w2 = "abc";
    assert_eq!(min_distance(w1.into(), w2.into()), 1);
}
