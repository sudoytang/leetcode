
#[allow(unused)]
pub struct Solution;

#[allow(unused)]
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::collections::HashSet;

        // constr-i: only alpha, number, whitespace, puncts in s, can intepret as Vec<u8>
        // use a slice of s to calculate the byte occurance
        let mut hs = HashSet::new();
        let bytes = s.as_bytes();
        let mut lower = 0;
        let mut upper = 0;
        let mut max_len = 0;
        while upper < s.len() {
            if hs.insert(bytes[upper]) {
                // ok
                upper += 1;
                max_len = max_len.max(upper - lower);
            } else {
                // already inserted
                while bytes[lower] != bytes[upper] {
                    let res = hs.remove(&bytes[lower]);
                    assert!(res);
                    lower += 1;
                }
                // at here the lower and upper points to same character
                // the set contains that character
                lower += 1;
                upper += 1;
                max_len = max_len.max(upper - lower);

            }
        }
        max_len as i32
    }
}