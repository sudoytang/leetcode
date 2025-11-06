use std::rc::Rc;
use std::cell::RefCell;
#[cfg(feature = "local")]
#[allow(unused)]
pub struct Solution;

#[allow(unused)]
use crate::TreeNode;
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            Some(n) => {
                {
                    let mut bm = n.borrow_mut();
                    let left = Self::invert_tree(bm.left.take());
                    let right = Self::invert_tree(bm.right.take());
                    bm.right = left;
                    bm.left = right;
                }
                Some(n)
            }
            None => None
        }
    }
}