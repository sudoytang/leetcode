use std::collections::HashSet;

#[allow(unused)]
struct Solution;
#[allow(unused)]
impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        let broken_charset: HashSet<_> = broken_letters.chars().collect();
        let splits = text
            .split(char::is_whitespace)
            .filter(|s| !s.is_empty());
        splits.clone().count() as i32 - splits
            .filter(|s| s.chars().any(|c| broken_charset.contains(&c)))
            .count() as i32
    }
}