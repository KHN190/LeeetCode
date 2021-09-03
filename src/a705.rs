// https://leetcode.com/problems/design-hashset/

// Design a HashSet without using any built-in hash table libraries.

// 0 <= key <= 10^6
// At most 10^4 calls will be made to add, remove, and contains.

pub struct MyHashSet {
    set: [bool; 1000001],
}

impl MyHashSet {
    pub fn new() -> Self {
        MyHashSet {
            set: [false; 1000001],
        }
    }

    pub fn add(&mut self, key: i32) {
        self.set[key as usize] = true;
    }

    pub fn remove(&mut self, key: i32) {
        self.set[key as usize] = false;
    }

    pub fn contains(&self, key: i32) -> bool {
        self.set[key as usize]
    }
}

#[test]
fn run() {
    let mut hashie = MyHashSet::new();
    hashie.add(1);
    hashie.add(2);
    assert!(hashie.contains(1));
    assert!(hashie.contains(2));
    assert!(!hashie.contains(3));

    hashie.remove(1);
    assert!(!hashie.contains(1));
}
