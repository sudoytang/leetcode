use std::collections::BinaryHeap;

use crate::listnode::ListNode;
#[allow(unused)]
pub struct Solution;
#[allow(unused)]
impl Solution {
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut min_heap = BinaryHeap::new();
        for i in 0..lists.len() {
            match lists[i].take() {
                Some(mut n) => {
                    lists[i] = n.next.take();
                    min_heap.push(NodeWrap(n, i));
                }
                None => {
                    lists[i] = None;
                }
            }
        }

        let mut new_head = None;
        let mut tail = &mut new_head;

        while let Some(NodeWrap(node, i)) = min_heap.pop() {
            tail.replace(node);
            tail = &mut tail.as_mut().unwrap().next;
            match lists[i].take() {
                Some(mut n) => {
                    lists[i] = n.next.take();
                    min_heap.push(NodeWrap(n, i));
                }
                None => {
                    lists[i] = None;
                }
            }
        }
        new_head
    }
}

struct NodeWrap(Box<ListNode>, usize);

impl PartialEq for NodeWrap {
    fn eq(&self, other: &Self) -> bool {
        other.0.val == self.0.val
    }
}

impl Eq for NodeWrap {}

impl PartialOrd for NodeWrap {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.0.val.partial_cmp(&self.0.val)
    }
}

impl Ord for NodeWrap {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.val.cmp(&self.0.val)
    }
}