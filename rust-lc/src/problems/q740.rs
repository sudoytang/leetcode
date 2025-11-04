use std::collections::BTreeMap;

use super::Solution;

impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        // nums: array
        // values: Map(i32 -> usize(count))
        let map = nums.into_iter().fold(BTreeMap::new(), |mut map, num| {
            *map.entry(num).or_insert(0) += 1;
            map
        });

        let mut last = i32::MIN;
        let mut v1 = 0;
        let mut v2 = 0;
        // current = max(v1, v2 + count * i)
        for (i, count) in map {
            let cur = if last < i - 1 {
                v1 + i * count
            } else {
                i32::max(v1, v2 + i * count)
            };
            last = i;
            v2 = v1;
            v1 = cur;
        }
        v1
    }
}