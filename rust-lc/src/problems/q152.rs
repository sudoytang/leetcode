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
    pub fn max_product(nums: Vec<i32>) -> i32 {
        // max[i]: the csubarray with max product of nums ending(incl) at ith
        // min[i]: the csubarray with min product of nums ending(incl) at ith
        // max[i]: if nums[i] > 0: max[i-1] * nums[i]
        //         if nums[i] < 0: min[i-1] * nums[i]
        //         or start from i

        let mut max = vec![0; nums.len()];
        let mut min = vec![0; nums.len()];
        max[0] = nums[0];
        min[0] = nums[0];
        for i in 1..max.len() {
            max[i] = nums[i].max(max[i-1] * nums[i]).max(min[i-1] * nums[i]);
            min[i] = nums[i].min(max[i-1] * nums[i]).min(min[i-1] * nums[i]);
        }
        max.into_iter().max().unwrap()
    }
}