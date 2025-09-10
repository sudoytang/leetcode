#[allow(unused)]
pub struct Solution;
#[allow(unused)]
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        const RED: usize = 0;
        const GREEN: usize = 1;
        const BLUE: usize = 2;
        let mut cf = [0; 3];
        for i in 0..nums.len() {
            let color =nums[i];
            cf[color as usize] += 1;
        }
        for i in 0..nums.len() {
            if cf[RED] > 0 {
                cf[RED] -= 1;
                nums[i] = RED as i32;
            } else if cf[GREEN] > 0 {
                cf[GREEN] -= 1;
                nums[i] = GREEN as i32;
            } else if cf[BLUE] > 0 {
                cf[BLUE] -= 1;
                nums[i] = BLUE as i32;
            } else {
                unreachable!()
            }
        }
    }
}