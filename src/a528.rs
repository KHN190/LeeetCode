// https://leetcode.com/problems/random-pick-with-weight/

// The probability of picking an index i is w[i] / sum(w).

use rand::{thread_rng, Rng};

#[derive(Default)]
pub struct Solution {
    probs: Vec<i32>, // accumulative sum
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    pub fn new(w: Vec<i32>) -> Self {
        let mut probs: Vec<i32> = vec![];
        for (i, c) in w.iter().enumerate() {
            if i == 0 {
                probs.push(*c);
            } else {
                probs.push(*c + probs[i - 1]);
            }
        }
        Self { probs }
    }

    pub fn pick_index(&self) -> i32 {
        let bound = self.probs[self.probs.len() - 1];
        let mut rnd = thread_rng();
        let r = rnd.gen_range(0..=bound);
        let i = match self.probs.binary_search(&r) {
            Ok(idx) => idx,
            Err(idx) => idx,
        };
        i as i32
    }
}

#[test]
fn run() {
    let s = Solution::new(vec![1, 3]);
    let e = s.pick_index();
    assert!(e == 0 || e == 1);
}
