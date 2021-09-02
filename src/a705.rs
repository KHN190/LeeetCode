// https://leetcode.com/problems/design-hashset/

// Design a HashSet without using any built-in hash table libraries.

// 0 <= key <= 10^6
// At most 10^4 calls will be made to add, remove, and contains.

#[allow(dead_code)]
struct MyHashSet {
    set: [bool; 1000001],
}

#[allow(dead_code)]
impl MyHashSet {
    fn new() -> Self {
        MyHashSet {
            set: [false; 1000001],
        }
    }

    fn add(&mut self, key: i32) {
        self.set[key as usize] = true;
    }

    fn remove(&mut self, key: i32) {
        self.set[key as usize] = false;
    }

    fn contains(&self, key: i32) -> bool {
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
