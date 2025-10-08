

use crate::listnode::ListNode;
#[allow(unused)]
pub struct Solution;
#[allow(unused)]
impl Solution {
    fn len(mut head: &Option<Box<ListNode>>) -> usize {
        let mut count = 0;
        while let Some(t) = head {
            count += 1;
            head = &t.next;
        }
        count
    }
    pub fn is_palindrome(mut head: Option<Box<ListNode>>) -> bool {
        if head.is_none() {
            return true;
        }

        let len = Self::len(&head);
        let step = (len - 1) / 2;
        let mut ptr = &mut head;

        for _ in 0..step {
            ptr = &mut ptr.as_mut().unwrap().next;
        }
        let lower_half = ptr.as_mut().unwrap().next.take();
        let lower_half = Self::reverse_list2(lower_half);
        let mut p1 = &head;
        let mut p2 = &lower_half;
        while let (Some(n1), Some(n2)) = (p1, p2) {
            if (n1.val != n2.val) {
                return false;
            }
            p1 = &n1.next;
            p2 = &n2.next;
        }
        return true;

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