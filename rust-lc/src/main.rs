pub mod problems;
pub mod treenode;
pub mod listnode;
#[allow(unused)]
use listnode::ListNode;
use treenode::TreeNode;
use crate::problems::Solution;
fn main() {
    let mut v = vec![
        vec![0, 1, 2, 0],
        vec![3, 4, 5, 2],
        vec![1, 3, 1, 5]
    ];

    Solution::set_zeroes(&mut v);
    println!("{:#?}", v);

}
