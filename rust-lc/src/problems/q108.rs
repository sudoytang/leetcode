#[cfg(feature = "local")]
#[allow(unused)]
pub struct Solution;

#[allow(unused)]
use crate::TreeNode;

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    fn sorted_array_to_bst_impl(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        match nums.len() {
            0 => None,
            1 => Some(Rc::new(RefCell::new(TreeNode::new(nums[0])))),
            n => {
                let split = n / 2;
                let left = &nums[..split];
                let mid = nums[split];
                let right = &nums[split+1..];
                let resnode = Rc::new(RefCell::new(TreeNode::new(mid)));
                {
                    let mut mutref = resnode.borrow_mut();
                    mutref.left = Self::sorted_array_to_bst_impl(left);
                    mutref.right = Self::sorted_array_to_bst_impl(right);
                }
                Some(resnode)
            }
        }
    }

    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::sorted_array_to_bst_impl(&nums)
    }
}