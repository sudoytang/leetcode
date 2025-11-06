use crate::TreeNode;
#[cfg(feature = "local")]
#[allow(unused)]
pub struct Solution;

#[allow(unused)]


use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    fn build_tree_impl(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        assert_eq!(preorder.len(), inorder.len());
        if preorder.len() == 0 {
            return None;
        }


        let pivot = preorder[0];
        let node = Rc::new(RefCell::new(TreeNode::new(pivot)));
        let idx = inorder.iter().position(|v| *v == pivot).unwrap();
        let ino_left = &inorder[..idx];
        let ino_right = &inorder[idx+1..];
        let pro_left = &preorder[1..(1+ino_left.len())];
        let pro_right = &preorder[(1+ino_left.len())..];
        {
            let mut bm = node.borrow_mut();
            bm.left = Self::build_tree_impl(pro_left, ino_left);
            bm.right = Self::build_tree_impl(pro_right, ino_right);
        }
        Some(node)
    }
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build_tree_impl(&preorder, &inorder)
    }
}