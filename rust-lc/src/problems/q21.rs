// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }


use crate::listnode::ListNode;
#[allow(unused)]
pub struct Solution;
#[allow(unused)]
impl Solution {
    pub fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(-1);
        let mut tail = &mut dummy.next;
        loop {
            match (list1, list2) {
                (None, None) => return dummy.next,
                (None, Some(mut n)) => {
                    list1 = None;
                    list2 = n.next.take();
                    tail.replace(n);
                    tail = &mut tail.as_mut().unwrap().next;
                }
                (Some(mut n), None) => {
                    list1 = n.next.take();
                    tail.replace(n);
                    tail = &mut tail.as_mut().unwrap().next;
                    list2 = None;
                }
                (Some(mut n1), Some(mut n2)) => {
                    if (n1.val <= n2.val) {
                        // pick n1
                        list1 = n1.next.take();
                        tail.replace(n1);
                        tail = &mut tail.as_mut().unwrap().next;
                        list2 = Some(n2);
                    } else {
                        // pick n2
                        list1 = Some(n1);
                        list2 = n2.next.take();
                        tail.replace(n2);
                        tail = &mut tail.as_mut().unwrap().next;
                    }
                }
            }
        }
    }
}