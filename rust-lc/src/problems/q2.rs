
use crate::listnode::ListNode;
#[allow(unused)]
struct Solution;
#[allow(unused)]
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut p1 = &l1;
        let mut p2 = &l2;
        let mut ldst: Option<Box<ListNode>> = None;
        let mut pdst = &mut ldst;
        let mut carry = 0;
        loop {
            match (p1.as_ref(), p2.as_ref()) {
                (None, None) => {
                    if carry != 0 {
                        pdst.replace(Box::new(ListNode::new(carry)));
                    }
                    return ldst;
                }
                (None, Some(n2)) => {
                    let sum = carry + n2.val;
                    carry = sum / 10;
                    pdst.replace(Box::new(ListNode::new(sum % 10)));
                    pdst = &mut pdst.as_mut().unwrap().next;
                    p2 = &n2.next;
                }
                (Some(n1), None) => {
                    let sum = carry + n1.val;
                    carry = sum / 10;
                    pdst.replace(Box::new(ListNode::new(sum % 10)));
                    pdst = &mut pdst.as_mut().unwrap().next;
                    p1 = &n1.next;
                }
                (Some(n1), Some(n2)) => {
                    let sum = carry + n1.val + n2.val;
                    carry = sum / 10;
                    pdst.replace(Box::new(ListNode::new(sum % 10)));
                    pdst = &mut pdst.as_mut().unwrap().next;
                    p1 = &n1.next;
                    p2 = &n2.next;
                }
            }
        }
    }
}