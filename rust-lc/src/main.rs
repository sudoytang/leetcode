use crate::problems::q1488;

pub mod problems;
pub mod treenode;
pub mod listnode;

fn main() {
    let _res = q1488::Solution::avoid_flood([1, 0, 2, 3, 0, 1, 2].into());
    println!("{_res:?}");
}
