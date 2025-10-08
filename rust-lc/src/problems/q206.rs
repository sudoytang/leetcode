
use std::ptr::NonNull;

use crate::listnode::ListNode;
#[allow(unused)]
struct Solution;
#[allow(unused)]
impl Solution {

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

    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            Some(n) => Some(Self::reverse_list_impl(n).0),
            None => None,
        }
    }

    pub fn reverse_list2(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        let mut curr = head;
        while let Some(mut n) = curr {
            let next = n.next.take();
            n.next = prev;
            prev = Some(n);
            curr = next;
        }

        prev
    }
}