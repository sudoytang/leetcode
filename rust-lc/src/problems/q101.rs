
use std::rc::Rc;
use std::cell::RefCell;
#[cfg(feature = "local")]
#[allow(unused)]
pub struct Solution;

#[allow(unused)]
use crate::TreeNode;
impl Solution {
    fn is_inv_eq(left: &Option<Rc<RefCell<TreeNode>>>, right: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (left,right) {
            (None, None) => true,
            (Some(nl), Some(nr)) => {
                nl.borrow().val == nr.borrow().val &&
                Self::is_inv_eq(&nl.borrow().left, &nr.borrow().right) &&
                Self::is_inv_eq(&nl.borrow().right, &nr.borrow().left)
            }
            _ => false,
        }
    }
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            Some(n) => {
                let bl = &n.borrow().left;
                let br = &n.borrow().right;
                Self::is_inv_eq(bl, br)
            }
            None => true
        }
    }
}