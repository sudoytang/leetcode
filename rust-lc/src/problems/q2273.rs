#[cfg(feature = "local")]
#[allow(unused)]
pub struct Solution;

#[allow(unused)]


impl Solution {
    fn count_cf(word: &str) -> [usize; 26] {
        let mut res = [0; 26];
        for &ch in word.as_bytes() {
            res[(ch - b'a') as usize] += 1;
        }
        res
    }
    pub fn remove_anagrams(words: Vec<String>) -> Vec<String> {
        let mut words = words;
        let mut cf = [0; 26];
        words.retain(|word| {
            let cur_cf = Self::count_cf(word);
            if cf == cur_cf {
                false
            } else {
                cf = cur_cf;
                true
            }
        });

        words
    }
}