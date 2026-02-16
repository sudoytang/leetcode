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
    pub fn minimum_difference(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut nums = nums;
        nums.sort_unstable();

        let mut min = i32::MAX;

        for i in 0..(nums.len() - (k-1)) {
            min = min.min(nums[i+k-1]-nums[i]);
        }

        min

    }
}