// insert, delete & lookup should be O(1)
// so we need val and the position both stored.

use rand::{thread_rng, Rng};
use std::collections::HashMap;

#[derive(Default)]
pub struct RandomizedSet {
    data: Vec<i32>,
    idx: HashMap<i32, usize>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert(&mut self, val: i32) -> bool {
        match self.idx.get(&val) {
            Some(_) => false,
            None => {
                self.idx.insert(val, self.data.len());
                self.data.push(val);
                true
            }
        }
    }

    pub fn remove(&mut self, val: i32) -> bool {
        match self.idx.get(&val) {
            None => false,
            Some(&i) => {
                self.data.swap_remove(i);
                if let Some(v) = self.data.get(i) {
                    self.idx.insert(*v, i);
                }
                self.idx.remove(&val);
                true
            }
        }
    }

    pub fn get_random(&self) -> i32 {
        let mut rnd = thread_rng();
        let i = rnd.gen_range(0..self.data.len());
        self.data[i]
    }
}

#[test]
fn run() {
    let mut obj = RandomizedSet::new();

    let r1 = obj.insert(0);
    let r2 = obj.insert(1);
    let r3 = obj.remove(0);
    let r4 = obj.insert(2);
    let r5 = obj.remove(1);

    assert_eq!(r1, true);
    assert_eq!(r2, true);
    assert_eq!(r3, true);
    assert_eq!(r4, true);
    assert_eq!(r5, true);
    assert_eq!(obj.get_random(), 2);
}
