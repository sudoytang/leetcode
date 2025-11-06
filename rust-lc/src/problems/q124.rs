#[cfg(feature = "local")]
#[allow(unused)]
pub struct Solution;

#[allow(unused)]
use crate::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

type N = Rc<RefCell<TreeNode>>;
type ON = Option<N>;

impl Solution {

    fn max_link_sum(node: &ON, max_path_sum: &mut i32) -> i32 {
        match node {
            Some(n) => {
                let l_link_sum = Self::max_link_sum(&n.borrow().left, max_path_sum);
                let r_link_sum = Self::max_link_sum(&n.borrow().right, max_path_sum);
                *max_path_sum = i32::max(*max_path_sum, l_link_sum + r_link_sum + n.borrow().val);
                (l_link_sum.max(r_link_sum) + n.borrow().val).max(0)
            }
            None => 0,
        }
    }

    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_path_sum = i32::MIN;
        Self::max_link_sum(&root, &mut max_path_sum);
        max_path_sum
    }
}