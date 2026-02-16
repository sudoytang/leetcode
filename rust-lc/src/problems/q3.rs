#[cfg(feature = "local")]
#[allow(unused)]
pub struct Solution;

#[cfg(feature = "local")]
#[allow(unused)]
use crate::ListNode;

#[cfg(feature = "local")]
#[allow(unused)]
use crate::TreeNode;


impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.len() == 0 {
            return 0;
        }
        let s = s.as_bytes();
        // inclusive exclusive bound lr
        let mut l = 0;
        let mut r = 0;

        let mut wordfreq = [0; 256];
        let mut max_len = 0;

        while r < s. len() {
            if wordfreq[s[r] as usize] == 0 {
                wordfreq[s[r] as usize] = 1;
                r += 1;
                max_len = max_len.max(r - l);
            } else /* not none, non-zero */ {
                wordfreq[s[l] as usize] -= 1;
                l += 1;
            }
        }

        max_len as i32

    }
}