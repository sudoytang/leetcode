use crate::TreeNode;
use super::Solution;

use std::rc::Rc;
use std::cell::RefCell;


type ON = Option<N>;
type N = Rc<RefCell<TreeNode>>;
type P = *const TreeNode;
enum NodeSearchStat {
    Nothing,
    FoundP,
    FoundQ,
    FoundBoth(ON),
}


impl Solution {

    fn lowest_common_ancestor_impl(node: &ON, p: P, q: P) -> NodeSearchStat {
        use NodeSearchStat::*;
        if node.is_none() {
            return Nothing;
        }

        let node = node.as_ref().unwrap();
        if node.as_ptr() as P == p {
            if matches!(Self::lowest_common_ancestor_impl(&node.borrow().left, p, q), FoundQ)
            || matches!(Self::lowest_common_ancestor_impl(&node.borrow().right, p, q), FoundQ) {
                return FoundBoth(Some(node.clone()));
            }
            return FoundP;
        }
        if node.as_ptr() as P == q {
            if matches!(Self::lowest_common_ancestor_impl(&node.borrow().left, p, q), FoundP)
            || matches!(Self::lowest_common_ancestor_impl(&node.borrow().right, p, q), FoundP) {
                return FoundBoth(Some(node.clone()));
            }
            return FoundQ;
        }
        let stat1 = Self::lowest_common_ancestor_impl(&node.borrow().left, p, q);
        let stat2 = Self::lowest_common_ancestor_impl(&node.borrow().right, p, q);
        match (stat1, stat2) {
            (Nothing, Nothing) => Nothing,
            (Nothing, FoundP) => FoundP,
            (Nothing, FoundQ) => FoundQ,
            (Nothing, FoundBoth(at)) => FoundBoth(at),
            (FoundP, Nothing) => FoundP,
            (FoundP, FoundQ) => FoundBoth(Some(node.clone())),
            (FoundP, FoundP) => unreachable!(),
            (FoundP, FoundBoth(_)) => unreachable!(),
            (FoundQ, Nothing) => FoundQ,
            (FoundQ, FoundP) => FoundBoth(Some(node.clone())),
            (FoundQ, FoundQ) => unreachable!(),
            (FoundQ, FoundBoth(_)) => unreachable!(),
            (FoundBoth(at), Nothing) => FoundBoth(at),
            (FoundBoth(_), FoundP) => unreachable!(),
            (FoundBoth(_), FoundQ) => unreachable!(),
            (FoundBoth(_), FoundBoth(_)) => unreachable!(),
        }
    }


    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if p.is_none() || q.is_none() {
            return None;
        }
        let p = p.unwrap().as_ptr() as P;
        let q = q.unwrap().as_ptr() as P;
        match Self::lowest_common_ancestor_impl(&root, p, q) {
            NodeSearchStat::FoundBoth(at) => at,
            _ => unreachable!()
        }
    }
}