use crate::TreeNode;
#[cfg(feature = "local")]
#[allow(unused)]
pub struct Solution;

#[allow(unused)]
use std::rc::Rc;
use std::cell::RefCell;

struct State {
    res: Vec<Vec<i32>>,
}

impl Default for State {
    fn default() -> Self {
        Self { res: Default::default() }
    }
}

impl State {
    pub fn traverse(&mut self, node: &Option<Rc<RefCell<TreeNode>>>, depth: usize) {
        if node.is_none() {
            return;
        }
        let n = node.as_ref().unwrap();
        if self.res.len() <= depth {
            self.res.resize_with(depth + 1, || Vec::new());
        }
        self.res[depth].push(n.borrow().val);
        self.traverse(&n.borrow().left, depth + 1);
        self.traverse(&n.borrow().right, depth + 1);
    }
}

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut state = State::default();
        state.traverse(&root, 0);
        state.res
    }
}