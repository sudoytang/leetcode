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

    fn reverse_list_impl(mut head: Box<ListNode>) -> (/* head */Box<ListNode>, /* tail */NonNull<ListNode>) {
        let next = head.next.take();
        match next {
            None => {
                let ptr = NonNull::from(head.as_mut());
                (head, ptr)
            }
            Some(n) => {
                let (new_head, mut tail) = Self::reverse_list_impl(n);
                let nn = NonNull::from(head.as_mut());
                unsafe {
                    tail.as_mut().next = Some(head);
                }
                (new_head, nn)
            }
        }


    }


    fn split_k(mut head: Option<Box<ListNode>>, k: i32) -> Result<(Box<ListNode>, Option<Box<ListNode>>), Option<Box<ListNode>>> {
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

    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        match Self::split_k(head, k) {
            Ok((l1, l2)) => {
                let (rev, mut tail) = Self::reverse_list_impl(l1);
                match Self::reverse_k_group(l2, k) {
                    None => Some(rev),
                    Some(n) => {
                        unsafe {
                            tail.as_mut().next.replace(n);
                        }
                        Some(rev)
                    }
                }
            }
            Err(l) => {
                l
            }
        }
    }
}
