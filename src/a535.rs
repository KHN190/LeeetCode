// https://leetcode.com/problems/encode-and-decode-tinyurl/

// Design a system that encodes and decodes short urls
//
// Implement the Solution class:
//  Solution() Initializes the object of the system.
//  String encode(String longUrl) Returns a tiny URL for the given longUrl.
//  String decode(String shortUrl) Returns the original long URL for the given shortUrl.
//      It is guaranteed that the given shortUrl was encoded by the same object.

use std::collections::hash_map::DefaultHasher;
use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
};

#[allow(dead_code)]
#[derive(Default)]
struct Codec {
    dict: HashMap<String, String>,
}

#[allow(dead_code, non_snake_case)]
impl Codec {
    fn new() -> Self {
        Default::default()
    }

    fn encode(&mut self, longURL: String) -> String {
        let hex_digest = {
            let mut s = DefaultHasher::new();
            longURL.hash(&mut s);
            let s = s.finish();
            // format the digit to hex string
            format!("{:x}", s)
        };
        self.dict.insert(hex_digest.clone(), longURL);
        format!("http://tinyurl.com/{}", hex_digest)
    }

    fn decode(&self, shortURL: String) -> String {
        let hex_digest = shortURL.replace("http://tinyurl.com/", "");
        if let Some(a) = self.dict.get(&hex_digest) {
            return a.clone();
        }
        return "".into();
    }
}

#[test]
fn run() {
    let url = "https://leetcode.com/a-silly-hash-function";

    let mut obj = Codec::new();
    let tiny = obj.encode(url.into());
    assert_eq!(tiny, "http://tinyurl.com/bdf99084eba41f40");

    let ans = obj.decode(tiny);
    assert_eq!(ans, url);
}
