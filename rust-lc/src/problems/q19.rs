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
#[cfg(feature = "local")]
#[allow(unused)]
pub struct Solution;

#[allow(unused)]

impl Solution {
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        // if we use safe Rust, it's not even possible to finish in single scan, which requires an immutable borrow and a mutable borrow.
        let len: usize = {
            let mut count = 0;
            let mut p = &head;
            while let Some(n) = p {
                count += 1;
                p = &n.next;
            }
            count
        };
        let mut mutp = &mut head;
        for _ in 0..(len - n as usize) {
            mutp = &mut mutp.as_mut().unwrap().next;
        }
        let taken = mutp.take();
        match taken {
            Some(n) => {
                *mutp = n.next;
            }
            None => {}
        }

        head
    }
}