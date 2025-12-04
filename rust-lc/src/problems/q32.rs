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
    pub fn longest_valid_parentheses(s: String) -> i32 {
        // ...( -> 0
        // .?() -> r[?] + 2
        // ..?) -> go left for r[?], if s[i - 1 - r[i-1]] == ( then r[i - 1 - r[i-1] - 1] + r[i-1] + 2

        let s = s.as_bytes();
        let mut r = vec![0; s.len()];
        for i in 0..s.len() {
            r[i] = match s[i] {
                b'(' => 0,
                b')' if i == 0 => 0,
                b')' if s[i-1] == b'(' => if i < 2 {2} else {r[i-2] + 2},
                b')' if s[i-1] == b')' => {
                    let left_r = r[i-1] as usize;
                    if /* i - 1 - left_r < 0 */ left_r + 1 > i {
                        0
                    } else if s[i - 1 - left_r] == b'(' {
                        if i - 1 - left_r == 0 {
                            left_r as i32 + 2
                        } else {
                            left_r as i32 + r[i - 1 - left_r - 1] + 2
                        }
                    } else if s[i - 1 - left_r] == b')' {
                        0
                    } else {
                        unreachable!()
                    }
                }
                _ => unreachable!()
            }
        }

        r.into_iter().max().unwrap_or(0)
    }
}