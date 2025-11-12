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
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let acc = (nums[0], 1);
        let (n, count) = nums.into_iter().skip(1).fold(acc, |(n, count), val| {
            if n == val {
                (n, count + 1)
            } else if count == 0 {
                (val, 1)
            } else {
                (n, count - 1)
            }
        });
        assert!(count > 0);
        n
    }
}