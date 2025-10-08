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

use std::ptr::NonNull;

use crate::listnode::ListNode;
#[allow(unused)]
pub struct Solution;
#[allow(unused)]
impl Solution {
    fn len(mut head: &Option<Box<ListNode>>) -> usize {
        let mut count = 0;
        while let Some(n) = head.as_ref() {
            count += 1;
            head = &n.next;
        }
        count
    }

    pub fn merge(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<(Box<ListNode>, NonNull<ListNode>)> {
        let mut dummy = ListNode::new(-1);
        let mut tail: &mut ListNode = &mut dummy;

        loop {
            match (list1, list2) {
                (None, None) => {
                    let nn = NonNull::from(tail);
                    match dummy.next {
                        Some(n) => return Some((n, nn)),
                        None => return None,
                    }
                }
                (None, Some(mut n)) => {
                    list1 = None;
                    list2 = n.next.take();
                    tail.next.replace(n);
                    tail = tail.next.as_mut().unwrap();
                }
                (Some(mut n), None) => {
                    list1 = n.next.take();
                    tail.next.replace(n);
                    tail = tail.next.as_mut().unwrap();
                    list2 = None;
                }
                (Some(mut n1), Some(mut n2)) => {
                    if (n1.val <= n2.val) {
                        // pick n1
                        list1 = n1.next.take();
                        tail.next.replace(n1);
                        tail = tail.next.as_mut().unwrap();
                        list2 = Some(n2);
                    } else {
                        // pick n2
                        list1 = Some(n1);
                        list2 = n2.next.take();
                        tail.next.replace(n2);
                        tail = tail.next.as_mut().unwrap();
                    }
                }
            }
        }
    }

    fn split_k(
        mut head: Option<Box<ListNode>>,
        k: usize,
    ) -> Result<(Box<ListNode>, Option<Box<ListNode>>), Option<Box<ListNode>>> {
        let mut mutp = &mut head;
        let mut l2 = None;
        for _ in 0..k {
            match mutp.as_mut() {
                Some(n) => {
                    mutp = &mut n.next;
                }
                None => {
                    return Err(head);
                }
            }
        }
        l2 = mutp.take();
        Ok((head.unwrap(), l2))
    }

    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let len = Self::len(&head);
        let mut src_dummy = ListNode::new(0);
        let mut dst_dummy = ListNode::new(0);
        src_dummy.next = head;
        let mut step = 1;

        while step <= len {
            let mut tail = NonNull::from(&mut dst_dummy);
            while src_dummy.next.is_some() {
                // get first list with length = step (which is sorted by last iteration)
                // get second list with length = step
                let op1 = match Self::split_k(src_dummy.next, step) {
                    Ok((left, right)) => {
                        src_dummy.next = right;
                        Some(left)
                    }
                    Err(left) => {
                        src_dummy.next = None;
                        left
                    }
                };

                let op2 = match Self::split_k(src_dummy.next, step) {
                    Ok((left, right)) => {
                        src_dummy.next = right;
                        Some(left)
                    }
                    Err(left) => {
                        src_dummy.next = None;
                        left
                    }
                };
                // at here the merge must give us valid result since src_dummy.next is not None
                let (head, new_tail) = Self::merge(op1, op2).expect("The merge must be valid!");
                unsafe {
                    tail.as_mut().next = Some(head);
                }
                tail = new_tail;
            }
            step *= 2;
            src_dummy.next = dst_dummy.next.take();
        }
        src_dummy.next
    }
}
