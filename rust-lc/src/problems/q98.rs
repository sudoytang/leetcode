use crate::TreeNode;
use super::Solution;

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone, Copy)]
struct Range(Option<i32>, Option<i32>);
impl Range {
    fn contains(&self, val: i32) -> bool {
        match (self.0, self.1) {
            (None, None) => true,
            (None, Some(r)) => val < r,
            (Some(l), None) => l < val,
            (Some(l), Some(r)) => l < val && val < r,
        }
    }

    fn bound_left(self, val: i32) -> Self {
        match self.0 {
            Some(v) => Self(Some(v.max(val)), self.1),
            None => Self(Some(val), self.1)
        }
    }

    fn bound_right(self, val: i32) -> Self {
        match self.1 {
            Some(v) => Self(self.0, Some(v.min(val))),
            None => Self(self.0, Some(val))
        }
    }
}

impl Solution {

    fn is_valid_bst_impl(root: &Option<Rc<RefCell<TreeNode>>>, range: Range) -> bool {
        match root {
            Some(n) => {
                let val = n.borrow().val;
                range.contains(val)
                && Self::is_valid_bst_impl(&n.borrow().left, range.bound_right(val))
                && Self::is_valid_bst_impl(&n.borrow().right, range.bound_left(val))
            }
            None => true
        }
    }

    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_valid_bst_impl(&root, Range(None, None))
    }
}