#[cfg(feature = "local")]
#[allow(unused)]
pub struct Solution;

use std::collections::HashMap;

#[cfg(feature = "local")]
#[allow(unused)]
use crate::ListNode;

#[cfg(feature = "local")]
#[allow(unused)]
use crate::TreeNode;

#[allow(unused)]
impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        if strs.len() == 0 {
            return 0;
        }
        // r[i, m, n]: the max length of subset of strs[0..i] with at most m 0s and i 1s
        let counts: Vec<_> = strs.into_iter().map(|s| s.chars().fold((0, 0), |(zeros, ones), c| match c {
            '0' => (zeros + 1, ones),
            '1' => (zeros, ones + 1),
            _ => unreachable!()
        })).collect();
        Memo::new(&counts).recur( counts.len(), m as usize, n as usize) as i32
    }
}

struct Memo<'a> {
    r: HashMap<(usize, usize, usize), usize>,
    counts: &'a Vec<(usize, usize)>
}

impl<'a> Memo<'a> {
    pub fn new(counts: &'a Vec<(usize, usize)>) -> Self {
        Self {
            r: Default::default(),
            counts,
        }
    }
    pub fn recur(&mut self, i: usize, m: usize, n: usize) -> usize {
        if i == 0 {
            return 0;
        }
        if let Some(&v) = self.r.get(&(i, m, n)) {
            return v;
        }
        // i > 0
        // if we do not choose here
        let cand1 = self.recur(i - 1, m, n);
        // if we choose
        let (zeros, ones) = self.counts[i - 1];
        let cand2 = if m >= zeros && n >= ones {
            self.recur(i - 1, m - zeros, n - ones) + 1
        } else {
            0
        };
        let res = cand1.max(cand2);
        self.r.insert((i, m, n), res);
        res
    }
}