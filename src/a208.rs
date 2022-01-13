// https://leetcode.com/problems/implement-trie-prefix-tree/

#[derive(Default)]
pub struct Trie {
    is_end: bool,                   // is this node an end of a word?
    nodes: [Option<Box<Trie>>; 26], // 26 alphabets
}

impl Trie {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert(&mut self, word: String) {
        let mut curr = self;
        for i in word.chars().map(|c| (c as u8 - 'a' as u8) as usize) {
            curr = curr.nodes[i].get_or_insert_with(|| Box::new(Trie::new()));
        }
        curr.is_end = true;
    }

    // true if word is in the trie
    pub fn search(&self, word: String) -> bool {
        // current pos in node
        let mut curr = self;
        for i in word.chars().map(|c| (c as u8 - 'a' as u8) as usize) {
            match curr.nodes[i].as_ref() {
                Some(node) => {
                    curr = node;
                }
                None => {
                    return false;
                }
            }
        }
        curr.is_end
    }

    // true if prefix is in the trie
    pub fn starts_with(&self, prefix: String) -> bool {
        let mut curr = self;
        for i in prefix.chars().map(|char| (char as u8 - 'a' as u8) as usize) {
            match curr.nodes[i].as_ref() {
                Some(node) => {
                    curr = node;
                }
                None => {
                    return false;
                }
            }
        }
        true
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */

#[test]
fn run() {
    let mut trie = Trie::new();
    trie.insert("world".into());
    trie.insert("walden".into());

    assert_eq!(trie.search("world".into()), true);
    assert_eq!(trie.search("word".into()), false);
    assert_eq!(trie.search("walden".into()), true);

    assert_eq!(trie.starts_with("world".into()), true);
    assert_eq!(trie.starts_with("wal".into()), true);
    assert_eq!(trie.starts_with("wore".into()), false);
}
