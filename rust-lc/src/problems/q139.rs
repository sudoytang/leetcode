#[cfg(feature = "local")]
#[allow(unused)]
pub struct Solution;

use std::collections::HashMap;

#[cfg(feature = "local")]
#[allow(unused)]
use crate::ListNode;

#[cfg(feature = "local")]
#[allow(unused)]
use crate::TreeNode;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        // r[i] -> bool : can the substr s[0..i] be built from word_dict?
        // r[0] = true (because we don' need anything for an empty string)
        // r[i]: if there is any word w s.t. s[0..i].ends_with(w) and r[i - w.len()] = true
        // returns r[s.len()]
        Memo::new(&s, &word_dict).recur(s.len())
    }
}


struct Memo<'a> {
    r: HashMap<usize, bool>,
    s: &'a str,
    dict: &'a Vec<String>
}

impl<'a> Memo<'a> {
    fn new(s: &'a str, dict: &'a Vec<String>) -> Self {
        Self {
            r: HashMap::new(),
            s,
            dict
        }
    }
    fn recur(&mut self, i: usize) -> bool {
        if i == 0 {
            return true;
        }
        if let Some(&f) = self.r.get(&i) {
            return f;
        }
        // not found
        let sub = &self.s[0..i];
        let result = self.dict.iter().any(|word| {
            sub.ends_with(word) && self.recur(i - word.len())
        });
        self.r.insert(i, result);
        result
    }
}