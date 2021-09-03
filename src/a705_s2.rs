// real hash table solution

// we can't create array of Vec more than 32 size,
// so we create Vec of Vec.

pub struct MyHashSet {
    data: Vec<Vec<i32>>, // 997 size: 10^6 ~= 997^2
}

impl MyHashSet {
    pub fn new() -> Self {
        let mut table: Vec<Vec<i32>> = vec![];
        for _ in 0..997 {
            let row: Vec<i32> = vec![];
            table.push(row);
        }
        MyHashSet { data: table }
    }

    pub fn add(&mut self, key: i32) {
        let h = self.hash(key) as usize;
        for val in &self.data[h] {
            if val == &key {
                return;
            }
        }
        self.data[h].push(key);
    }

    pub fn remove(&mut self, key: i32) {
        let h = self.hash(key) as usize;
        let i = self.data[h].iter().position(|&x| x == key);
        if i != None {
            self.data[h].remove(i.unwrap());
        }
    }

    pub fn contains(&self, key: i32) -> bool {
        let h = self.hash(key) as usize;
        let i = self.data[h].iter().position(|&x| x == key);
        i != None
    }

    fn hash(&self, key: i32) -> i32 {
        key % 997
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
