use crate::TreeNode;
#[cfg(feature = "local")]
#[allow(unused)]
pub struct Solution;

#[allow(unused)]


use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn flatten_impl(root: &mut Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }
        let mr: &mut Rc<RefCell<TreeNode>> = root.as_mut().unwrap();
        let mut bm = mr.borrow_mut();
        let mut left = bm.left.take();
        let mut right = bm.right.take();
        let left_tail = Self::flatten_impl(&mut left);
        let right_tail = Self::flatten_impl(&mut right);
        match (left_tail, right_tail) {
            (None, None) => Some(mr.clone()),
            (Some(l), None) => {
                bm.right = left;
                Some(l)
            }
            (None, Some(r)) => {
                bm.right = right;
                Some(r)
            }
            (Some(l), Some(r)) => {
                bm.right = left;
                l.borrow_mut().right = right;
                Some(r)
            }
        }
    }
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        Self::flatten_impl(root);
    }
}