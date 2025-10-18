
#[allow(unused)]
struct Solution;
#[allow(unused)]


impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let k = (nums.len() as i32 + k) as usize % nums.len();
        nums.reverse();
        let (sl1, sl2) = nums.split_at_mut(k);
        sl1.reverse();
        sl2.reverse();
    }
}