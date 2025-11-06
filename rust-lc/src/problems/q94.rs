use crate::treenode::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;
#[allow(unused)]
pub struct Solution;
#[allow(unused)]

impl Solution {

    fn traverse(sink: &mut Vec<i32>, node: &Option<Rc<RefCell<TreeNode>>>) {
        match node {
            Some(rc) => {
                Self::traverse(sink, &rc.borrow().left);
                sink.push(rc.borrow().val);
                Self::traverse(sink, &rc.borrow().right);
            }
            None => {}
        }
    }

    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut sink = Vec::new();
        Self::traverse(&mut sink, &root);
        sink
    }
}