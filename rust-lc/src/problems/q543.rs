
use std::rc::Rc;
use std::cell::RefCell;
use crate::TreeNode;
use super::Solution;
impl Solution {
    pub fn compute(root: &Option<Rc<RefCell<TreeNode>>>) -> (usize/* max_height */, usize/* diameter */) {
        match root {
            Some(n) => {
                let (lh, ld) = Self::compute(&n.borrow().left);
                let (rh,rd) = Self::compute(&n.borrow().right);
                (lh.max(rh) + 1, (lh + rh + 1).max(ld).max(rd))
            }
            None => (0, 0)
        }
    }
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::compute(&root).1 as i32 - 1
    }
}