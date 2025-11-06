use std::collections::HashMap;

#[cfg(feature = "local")]
#[allow(unused)]
pub struct Solution;

#[allow(unused)]
impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let it1 = pattern.chars();
        let it2 = s.split(' ');
        if it2.count() != it1.count() {
            return false;
        }
        pattern.chars().zip(s.split(' ')).fold((HashMap::new(), HashMap::new(), false),
        |(mut map1, mut map2, mut failed), (ch, word)| {
            if failed {
                return (map1, map2, failed);
            }
            if *map1.entry(ch).or_insert(word) != word {
                failed = true;
            }
            if *map2.entry(word).or_insert(ch) != ch {
                failed = true;
            }

            (map1, map2, failed)
        }).2 == false
    }
}