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
    pub fn min_number_operations(target: Vec<i32>) -> i32 {
        let (count, max) = target.into_iter().fold((0, 0), |(count, max), cur| {
            (count + 0.max(max - cur), cur)
        });

        count + max
    }
}