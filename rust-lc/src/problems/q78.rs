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
    pub fn recur(nums: &[i32], cur: &mut Vec<i32>, i: usize, sink: &mut Vec<Vec<i32>>) {
        if i == nums.len() {
            // finished
            sink.push(cur.clone());
            return;
        }
        // not choosing this element
        Self::recur(nums, cur, i + 1, sink);
        cur.push(nums[i]);
        Self::recur(nums, cur, i + 1, sink);
        cur.pop();
    }
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut cur = Vec::new();
        Self::recur(&nums, &mut cur, 0, &mut res);
        res
    }
}