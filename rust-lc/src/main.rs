pub mod problems;
pub mod treenode;
pub mod listnode;

fn main() {

    // let u = problems::q1733::Solution::minimum_teachings(2, [[1].into(),[2].into(),[1,2].into()].into(), [[1,2].into(),[1,3].into(),[2,3].into()].into());
    let res = problems::q15::Solution::three_sum([-1,0,1,2,-1,-4].into());
    println!("{res:?}");
}
