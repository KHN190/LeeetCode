// https://leetcode.com/problems/design-hashset/

// Design a HashSet without using any built-in hash table libraries.

// 0 <= key <= 10^6
// At most 10^4 calls will be made to add, remove, and contains.

#[allow(dead_code)]
struct MyHashSet {
    set: Vec<u8>,
}

#[allow(dead_code)]
impl MyHashSet {
    fn new() -> Self {
        let mut data = Vec::with_capacity(100001 as usize);
        for _ in 0..100001 {
            data.push(0);
        }
        MyHashSet { set: data }
    }

    fn add(&mut self, key: i32) {
        self.set[key as usize] = 1;
    }

    fn remove(&mut self, key: i32) {
        self.set[key as usize] = 0;
    }

    fn contains(&self, key: i32) -> bool {
        if self.set[key as usize] == 1 {
            return true;
        }
        return false;
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
