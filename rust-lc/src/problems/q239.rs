use std::collections::VecDeque;

use super::Solution;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut deque: VecDeque<(usize, i32)> = VecDeque::new();
        let mut res = Vec::with_capacity(nums.len() - (k - 1));
        for (i, v) in nums.into_iter().enumerate() {
            while let Some(&(_, val)) = deque.back() {
                // the deque is a queue that has decending order
                if val <= v {
                    deque.pop_back(); // 5 3 1, +4 => 5, 4
                } else {
                    break;
                }
            }
            deque.push_back((i, v));

            if deque.front().unwrap().0 + k <= i {
                deque.pop_front();  // this is out of the sliding window
            }

            if i + 1 >= k {
                res.push(deque.front().unwrap().1);
            }
        }

        res
    }
}