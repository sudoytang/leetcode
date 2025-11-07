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
    pub fn get_letters(digit: u8) -> &'static str {
        let digit = digit - b'0';
        match digit {
            2 => "abc",
            3 => "def",
            4 => "ghi",
            5 => "jkl",
            6 => "mno",
            7 => "pqrs",
            8 => "tuv",
            9 => "wxyz",
            _ => unreachable!()
        }
    }
    pub fn recur(digits: &[u8], i: usize, cur: &mut String, sink: &mut Vec<String>) {
        if i == digits.len() {
            sink.push(cur.clone());
            return;
        }
        let letters = Self::get_letters(digits[i]);
        for letter in letters.chars() {
            cur.push(letter);
            Self::recur(digits, i + 1, cur, sink);
            cur.pop();
        }
    }
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut res = Vec::new();
        Self::recur(digits.as_bytes(), 0, &mut String::new(), &mut res);
        res
    }
}