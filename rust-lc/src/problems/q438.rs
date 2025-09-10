
#[allow(unused)]
pub struct Solution;

#[allow(unused)]
impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        if p.len() > s.len() {
            return [].into()
        }
        // constr-i: only lowercase alpha in s and p
        // use a slice of s to calculate the byte occurance
        let s = s.as_bytes();
        let p = p.as_bytes();
        let p_wf = p.iter().copied().fold([0; 26], |mut acc, b| {
            assert!((b'a'..=b'z').contains(&b));
            acc[(b - b'a') as usize] += 1;
            acc
        });
        let mut res = Vec::new();

        let mut lower = 0;
        let p_len = p.len();
        let mut upper = p_len;
        let mut slice_wf = s[0..p_len].iter().copied().fold([0; 26], |mut acc, b| {
            assert!((b'a'..=b'z').contains(&b));
            acc[(b - b'a') as usize] += 1;
            acc
        });

        while upper < s.len() {
            if slice_wf == p_wf {
                res.push(lower as i32);
            }

            slice_wf[(s[lower] - b'a') as usize] -= 1;
            slice_wf[(s[upper] - b'a') as usize] += 1;
            lower += 1;
            upper += 1;
        }
        if slice_wf == p_wf {
            res.push(lower as i32);
        }
        res
    }
}