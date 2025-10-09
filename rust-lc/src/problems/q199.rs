use crate::TreeNode;
use super::Solution;


use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn right_side_view_impl(res: &mut Vec<i32>, root: &Option<Rc<RefCell<TreeNode>>>, level: usize) {
        if root.is_none() {
            return;
        }
        let node = root.as_ref().unwrap();
        if res.len() <= level {
            res.reserve( 2 * res.len());
            res.resize(level + 1, 0);
        }
        res[level] = node.borrow().val;
        Self::right_side_view_impl(res, &node.borrow().left, level + 1);
        Self::right_side_view_impl(res, &node.borrow().right, level + 1);

    }
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();
        Self::right_side_view_impl(&mut res, &root, 0);
        res
    }
}