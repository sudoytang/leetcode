use super::Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut last_1: i32 = 0;
        let mut last_2 = 0;
        for num in nums {
            let current = i32::max(last_1, last_2 + num);
            last_2 = last_1;
            last_1 = current;
        }

        last_1
    }
}