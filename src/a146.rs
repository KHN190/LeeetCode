// https://leetcode.com/problems/lru-cache/
// design LRU cache

use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct LRUCache {
    data: HashMap<i32, i32>,
    lr: Vec<i32>, // least used elem is at the end
    capacity: i32,
}

impl LRUCache {
    // init with capacity
    pub fn new(capacity: i32) -> Self {
        let mut cache: LRUCache = Default::default();
        cache.capacity = capacity;
        cache
    }

    // return the val or -1
    // O(1)
    pub fn get(&mut self, key: i32) -> i32 {
        if self.data.contains_key(&key) {
            self.touch(key);
        }
        *self.data.get(&key).unwrap_or(&-1)
    }

    // update if key exists
    // otherwise, add new key
    // and remove least used key
    // O(1)
    pub fn put(&mut self, key: i32, value: i32) {
        self.data.insert(key, value);
        self.touch(key);
        // evict
        if self.lr.len() > self.capacity as usize {
            self.data.remove(&self.lr.pop().unwrap());
        }
    }

    fn touch(&mut self, key: i32) {
        match self.lr.iter().position(|x| *x == key) {
            Some(i) => {
                self.lr.remove(i);
            }
            None => {}
        };
        self.lr.insert(0, key);
    }
}

#[test]
pub fn run() {
    let mut cache: LRUCache = LRUCache::new(2);
    cache.put(1, 1);
    cache.put(2, 2);
    cache.get(1);
    cache.put(3, 3);

    assert_eq!(cache.lr, vec![3, 1]);
}
