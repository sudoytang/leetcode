#[cfg(feature = "local")]
#[allow(unused)]
pub struct Solution;

#[allow(unused)]
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // https://leetcode.cn/problems/two-sum/
        // given nums and target, return (index(x), index(y)) s.t. x + y = target and x, y in nums
        use std::collections::HashMap;
        let mut map = HashMap::new();
        for (i, num) in nums.into_iter().enumerate() {
            if let Some(&idx) = map.get(&(target - num)) {
                return vec![i as i32, idx as i32];
            }
            map.insert(num, i);
        }
        unreachable!()
    }
}