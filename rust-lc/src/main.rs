use crate::problems::{q394};

pub mod problems;
pub mod treenode;
pub mod listnode;

fn main() {
    let res = q394::Solution::decode_string("3[a2[c]]".into());
    println!("{res}");
}
