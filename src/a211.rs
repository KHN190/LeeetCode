// https://leetcode.com/problems/design-add-and-search-words-data-structure/

// note: '.' can be matched with any letter.

use crate::types::Trie;

#[derive(Default)]
pub struct WordDictionary {
    trie: Trie,
}

impl WordDictionary {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_word(&mut self, word: String) {
        self.trie.insert(&word);
    }

    pub fn search(&self, word: String) -> bool {
        if word.is_empty() {
            return false;
        }
        self.search_trie(&self.trie, &word)
    }

    fn search_trie(&self, trie: &Trie, word: &String) -> bool {
        if word.is_empty() {
            return trie.is_end;
        }
        // get first char
        let c = word.chars().nth(0).unwrap();
        let next_word: &str = &word.as_str()[1..];

        if c == '.' {
            // any char
            for node in &trie.nodes {
                if let Some(child) = node {
                    if self.search_trie(&child, &String::from(next_word)) {
                        return true;
                    }
                }
            }
        } else {
            // normal char
            let i = c as u8 - 'a' as u8;
            let node = &trie.nodes[i as usize];
            if let Some(child) = node {
                return self.search_trie(&child, &String::from(next_word));
            }
        }
        false
    }
}

#[test]
fn run() {
    let word = String::from("hello");
    let mut obj = WordDictionary::new();
    obj.add_word(word.clone());

    assert!(obj.search(word.clone()));
    assert!(obj.search("h..lo".into()))
}
