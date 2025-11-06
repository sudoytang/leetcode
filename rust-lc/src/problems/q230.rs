use crate::TreeNode;
#[cfg(feature = "local")]
#[allow(unused)]
pub struct Solution;

#[allow(unused)]


use std::rc::Rc;
use std::cell::RefCell;


impl Solution {
    fn kth_smallest_impl(counter: &mut i32, node: &Option<Rc<RefCell<TreeNode>>>, k: i32) -> Option<i32> {
        match node {
            Some(n) => {
                if let Some(val) = Self::kth_smallest_impl(counter, &n.borrow().left, k) {
                    return Some(val);
                }
                *counter += 1;
                if *counter == k {
                    return Some(n.borrow().val);
                }
                return Self::kth_smallest_impl(counter, &n.borrow().right, k);

            }
            None => None,
        }
    }
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut counter = 0;
        Self::kth_smallest_impl(&mut counter, &root, k).unwrap()
    }
}