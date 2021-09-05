// https://leetcode.com/problems/edit-distance/

// Edit distance from s1 to s2

pub fn min_distance(word1: String, word2: String) -> i32 {
    let (w1, w2) = if word1.len() > word2.len() {
        (&word2, &word1)
    } else {
        (&word1, &word2)
    };

    // @warn I still don't understand why init with this
    // single array saves memory but takes more run time
    let mut res: Vec<i32> = (0..w1.len() as i32 + 1).collect();
    // as_bytes() saves memory than chars()
    for (i2, c2) in w2.as_bytes().iter().enumerate() {
        let mut dist: Vec<i32> = vec![i2 as i32 + 1];
        for (i1, c1) in w1.as_bytes().iter().enumerate() {
            if c1 == c2 {
                // if chars at cur pos in 2 words are same
                // keep same edit dist from prev pos
                dist.push(res[i1]);
            } else {
                // otherwise, find prev min + 1
                dist.push(1 + res[i1].min(res[i1 + 1]).min(dist[dist.len() - 1]));
            }
        }
        res = dist;
    }
    res[res.len() - 1]
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
