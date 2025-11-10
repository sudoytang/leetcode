#[cfg(feature = "local")]
#[allow(unused)]
pub struct Solution;

#[cfg(feature = "local")]
#[allow(unused)]
use crate::ListNode;

#[cfg(feature = "local")]
#[allow(unused)]
use crate::TreeNode;

#[allow(unused)]
impl Solution {
    fn is_palindrome(s: &str) -> bool {
        s.chars().zip(s.chars().rev()).all(|(a, b)| a == b)
    }
    fn recur(start: usize, end_incl: usize, s: &str, cur: &mut Vec<String>, sink: &mut Vec<Vec<String>>) {
        if end_incl == s.len() {
            sink.push(cur.clone());
            return;
        }

        // split here
        let sub = &s[start..=end_incl];
        if Self::is_palindrome(sub) {
            cur.push(sub.into());
            Self::recur(end_incl + 1, end_incl + 1, s, cur, sink);
            cur.pop();
        }
        // do not split here
        if end_incl < s.len() - 1 {
            Self::recur(start, end_incl + 1, s, cur, sink);
        }
    }

    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut res = Vec::new();
        Self::recur(0, 0, &s, &mut Vec::new(), &mut res);
        res
    }
}