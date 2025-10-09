use crate::problems::{q234};

pub mod problems;
pub mod treenode;
pub mod listnode;
use listnode::ListNode;
use treenode::TreeNode;
fn main() {
    let head = Some(Box::new(ListNode::new_with(1, ListNode::new_with(0, ListNode::new(1)))));
    let res = q234::Solution::is_palindrome(head);
    println!("{res}");
}
