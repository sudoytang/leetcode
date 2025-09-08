
#[allow(unused)]
struct Solution;
use std::collections::HashSet;

#[allow(unused)]
impl Solution {

    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut nums: HashSet<i32> = nums.into_iter().collect();
        let mut streak = 0;
        for &num in nums.iter() {
            if nums.contains(&(num - 1)) {
                // already calculated
                continue;
            }
            let mut cur = num;
            let mut cur_streak = 1;
            while nums.contains(&(cur + 1)) {
                cur += 1;
                cur_streak += 1;
            }
            streak = streak.max(cur_streak);
        }
        streak
    }
}