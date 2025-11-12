#[cfg(feature = "local")]
#[allow(unused)]
pub struct Solution;

use std::collections::HashMap;

#[cfg(feature = "local")]
#[allow(unused)]
use crate::ListNode;

#[cfg(feature = "local")]
#[allow(unused)]
use crate::TreeNode;

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum: i32 = nums.iter().sum();
        if sum % 2 != 0 {
            // it's impossible to partition into two parts with same sum
            return false;
        }
        let sum = sum / 2;
        // this is now a 0-1 knapsack
        // r[i, val]: can I use the first i nums to get val?
        Memo::new(&nums).recur(nums.len(), sum)
    }
}

struct Memo<'a> {
    r: HashMap<(usize, i32), bool>,
    nums: &'a Vec<i32>,

}

impl<'a> Memo<'a> {
    fn new(nums: &'a Vec<i32>) -> Self {
        Self {
            r: Default::default(),
            nums
        }
    }

    fn recur(&mut self, i: usize, val: i32) -> bool {
        if val == 0 {
            return true;
        }
        if i == 0 {
            return false;
        }
        if let Some(&f) = self.r.get(&(i, val)) {
            return f;
        }
        // not choosing i-1
        let res1 = self.recur(i - 1, val);
        // choose i-1
        let cur = self.nums[i-1];
        let res2 = if cur <= val {
            self.recur(i-1, val - cur)
        } else {
            false
        };
        self.r.insert((i, val), res1 || res2);
        res1 || res2
    }
}