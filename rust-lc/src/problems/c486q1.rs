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
    pub fn minimum_prefix_length(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut acc = 1;
        for i in (0..(len-1)).rev() {
            if nums[i] >= nums[i + 1] {
                break;
            }
            acc += 1;
        }

        (len - acc) as i32
    }
}
