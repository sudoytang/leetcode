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
    pub fn min_pair_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let min = nums.iter().copied().zip(nums.iter().copied().rev()).max_by_key(|(a,b)| a + b);
        min.map(|(a, b)| a + b).unwrap()
    }
}
