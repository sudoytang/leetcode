
#[allow(unused)]
pub struct Solution;

#[allow(unused)]
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut idst = 0;
        for isrc in 0..nums.len() {
            if nums[isrc] != 0 {
                nums[idst] = nums[isrc];
                idst += 1;
            }
        }
        for i in idst..nums.len() {
            nums[i] = 0;
        }
    }
}