
#[allow(unused)]
pub struct Solution;
#[allow(unused)]
impl Solution {
    fn count_freq(s: &str) -> [usize; 26] {
        let mut res = [0; 26];
        for b in s.bytes() {
            assert!((b'a'..=b'z').contains(&b));
            res[(b - b'a') as usize] += 1;
        }
        res
    }
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        // constr-i: only [a-z] allowed
        // constr-o: return in any order
        use std::collections::HashMap;
        let mut map: HashMap<[usize; 26], Vec<String>> = HashMap::new();
        for (f, s) in strs.into_iter().map(|s| (Self::count_freq(&s), s)) {
            map.entry(f)
                .or_default()
                .push(s);
        }
        map.into_values().collect()
    }
}