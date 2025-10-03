pub mod problems;
pub mod treenode;
pub mod listnode;

fn main() {

    let res = problems::q287::Solution::find_duplicate([2,5,9,6,9,3,8,9,7,1].into());
    println!("{res}");
}
