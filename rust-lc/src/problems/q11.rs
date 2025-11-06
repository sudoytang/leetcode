#[cfg(feature = "local")]
#[allow(unused)]
pub struct Solution;

#[allow(unused)]

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut ileft = 0;
        let mut iright = height.len() - 1;
        let mut max = 0;
        while ileft < iright {
            max = max.max((iright - ileft) as i32 * i32::min(height[ileft], height[iright]));
            if height[ileft] < height[iright] {
                ileft += 1;
            } else {
                iright -= 1;
            }
        }
        max
    }
}