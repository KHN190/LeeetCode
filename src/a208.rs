// https://leetcode.com/problems/implement-trie-prefix-tree/

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */

#[test]
fn run() {
    use crate::types::Trie;

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
