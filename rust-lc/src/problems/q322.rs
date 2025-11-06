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

#[allow(unused)]
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        // r[i, amount] = r[i-1, amount].max(r[i, amount-coins[i]] + 1)
        Memo::new().coin_change(&coins, amount, coins.len() as isize - 1).unwrap_or(-1)
    }
}

pub struct Memo {
    map: HashMap<(i32, usize), Option<i32>>
}

impl Memo {
    pub fn new() -> Self {
        Self {
            map: HashMap::new()
        }
    }

    pub fn coin_change(&mut self, coins: &Vec<i32>, amount: i32, n: isize) -> Option<i32> {
        if n < 0 {
            if amount == 0 {
                return Some(0);
            } else {
                return None;
            }
        }
        let n = n as usize;  // now it's safe
        let key = (amount, n);
        if let Some(&cached) = self.map.get(&key) {
            cached
        } else if amount < coins[n] {
            let res = self.coin_change(coins, amount, n as isize - 1);
            self.map.insert(key, res);
            res
        } else {
            let a = self.coin_change(coins, amount, n as isize - 1);
            let b = self.coin_change(coins, amount - coins[n], n as isize).map(|v| v + 1);
            let res = a.zip(b).map(|(a, b)| a.min(b)).or(a).or(b);
            self.map.insert(key, res);
            res
        }
    }
}