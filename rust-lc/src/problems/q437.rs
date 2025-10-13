use super::Solution;
use crate::TreeNode;

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;


struct Memo {
    // the presum of current path vs. count
    presum_count: HashMap<i64, i64>,
}

impl Memo {
    fn new() -> Self {
        Memo {
            presum_count: HashMap::from([(0, 1)])
        }
    }

    fn dfs(&mut self, node: &Option<Rc<RefCell<TreeNode>>>, target_sum: i64, mut presum: i64) -> i64 {
        // presum is the current sum of [root, this node)

        if node.is_none() {
            return 0;
        }
        let node = node.as_ref().unwrap();
        presum += node.borrow().val as i64;
        // now presum is the current sum of [root, this node]
        let mut path_cnt = *self.presum_count.entry(presum - target_sum).or_default();
        // path_cnt is now the number of path with presum = (presum - target_sum)
        // so that from the node whose presum(that) = (presum(this) - target_sum)
        // to this node, the path sum is presum(this) - presus(that)
        // = presum(this) - presum(this) -(-target_sum) = target_sum
        // thus we find the count of path if this is the leaf node.
        *self.presum_count.entry(presum).or_default() += 1;
        path_cnt += self.dfs(&node.borrow().left, target_sum, presum);
        path_cnt += self.dfs(&node.borrow().right, target_sum, presum);
        // if this is not the leaf node, we will add counting for sub nodes.
        *self.presum_count.entry(presum).or_default() -= 1;
        path_cnt
    }
}

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        let mut memo = Memo::new();
        memo.dfs(&root, target_sum as i64, 0) as i32
    }
}