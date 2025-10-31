use std::collections::HashSet;

use super::Solution;

use crate::ListNode;


impl Solution {
    pub fn modified_list(nums: Vec<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        dummy.next = head;
        let set: HashSet<_> = nums.into_iter().collect();


        let mut i = &mut dummy.next;

        loop {
            if i.is_none() {
                return dummy.next;
            }
            // i is Some
            let mut cur = i.take().unwrap();
            let next = cur.next.take();
            if set.contains(&cur.val) {
                *i = next;
            } else {
                cur.next = next;
                i.replace(cur);
                i = &mut i.as_mut().unwrap().next;
            }
        }
    }
}