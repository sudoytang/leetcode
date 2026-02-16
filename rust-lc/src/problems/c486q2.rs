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
    pub fn rotate_elements(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut rot: Vec<i32> = nums.iter().copied().filter(|n| *n >= 0).collect();
        let rot_len = rot.len();
        if rot_len == 0 {
            return nums
        }
        rot.rotate_left(k as usize % rot_len);

        let mut p = 0;
        let mut nums = nums;
        nums.iter_mut().for_each(|n| {
            if *n >= 0 {
                *n = rot[p];
                p += 1;
            }
        });
        assert_eq!(p, rot.len());
        nums
    }
}
