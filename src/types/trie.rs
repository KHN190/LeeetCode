// # 208
// implement a trie

#[derive(Default)]
pub struct Trie {
    is_end: bool,                   // is this node an end of a word?
    nodes: [Option<Box<Trie>>; 26], // 26 alphabets
}

#[allow(dead_code)]
impl Trie {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert(&mut self, word: &String) {
        let mut curr = self;
        for i in word.chars().map(|c| (c as u8 - 'a' as u8) as usize) {
            curr = curr.nodes[i].get_or_insert_with(|| Box::new(Trie::new()));
        }
        curr.is_end = true;
    }

    // true if word is in the trie
    pub fn search(&self, word: &String) -> bool {
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
    pub fn starts_with(&self, prefix: &String) -> bool {
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
