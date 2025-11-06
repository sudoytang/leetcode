#[cfg(feature = "local")]
#[allow(unused)]
pub struct Solution;

#[cfg(feature = "local")]
#[allow(unused)]
use crate::ListNode;

#[cfg(feature = "local")]
#[allow(unused)]
use crate::TreeNode;


impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        // let min_cost[i] be the cost to climb to ith stair
        // then min_cost[i] =
        //      0 if i < 0, i = 0, i = 1
        //      min(min_cost[i-1] + cost[i-1], min_cost[i-2] + cost[i-2])

        if cost.len() < 2 {
            return 0;
        }
        let mut v2 = 0;   // min_cost[i-2]
        let mut v1 = 0;       // min_cost[i-1];

        for i in 2..=cost.len() {
            let cur = i32::min(v2 + cost[i - 2], v1 + cost[i - 1]);
            v2 = v1;
            v1 = cur;
        }

        v1
    }
}