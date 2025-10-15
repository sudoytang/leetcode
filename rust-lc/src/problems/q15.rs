use std::collections::HashSet;

use super::Solution;

impl Solution {

    pub fn two_sum_q15(nums: &[i32], target: i32) -> HashSet<(i32, i32)> {
        let mut map = HashSet::new();
        let mut res = HashSet::new();
        for &num in nums {
            if map.contains(&(target - num)) {
                res.insert((i32::min(num, target - num), i32::max(num, target - num)));
            }
            map.insert(num);
        }
        res
    }
    pub fn three_sum_1(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = HashSet::new();
        for (i, x) in nums.iter().copied().enumerate() {
            for (min, max) in Self::two_sum_q15(&nums[i+1..], -x) {
                let r1 = i32::min(x, min);
                let r3 = i32::max(x, max);
                let r2 = if r1 == x { min } else if r3 == x { max } else { x };
                res.insert(vec![r1, r2, r3]);
            }
        }
        res.into_iter().collect()
    }
}

#[allow(unused)]
impl Solution {
    pub fn three_sum_2(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return [].into()
        }
        nums.sort_unstable();
        let mut res = Vec::new();
        let mut last_numi = i32::MAX;
        for i in 0..nums.len() - 2 {
            if nums[i] == last_numi {
                continue;
            }
            last_numi = nums[i];
            let mut last_numj = i32::MAX;
            for j in i+1..nums.len() - 1 {
                if last_numj == nums[j] {
                    continue;
                }
                last_numj = nums[j];
                let target = -(nums[i] + nums[j]);
                if let Ok(k) = nums[j+1..].binary_search(&target) {
                    res.push(vec![nums[i], nums[j], nums[j + 1 + k]]);
                }
            }
        }
        res
    }
}

#[allow(unused)]
impl Solution {

    pub fn two_sum_on_sorted(nums: &[i32], target: i32, sink: &mut Vec<Vec<i32>>)  {
        let mut l = 0;
        let mut r = nums.len() - 1;
        while l < r {
            if l > 0 && nums[l] == nums[l-1] {
                l += 1;
                continue;
            }
            if r < nums.len() -1 && nums[r] == nums[r+1] {
                r -= 1;
                continue;
            }
            let vl = nums[l];
            let vr = nums[r];
            let res = nums[l] + nums[r];
            match res.cmp(&target) {
                std::cmp::Ordering::Less => l += 1,
                std::cmp::Ordering::Equal => {
                    sink.push(vec![-target, nums[l], nums[r]]);
                    l += 1;
                    r -= 1;
                }
                std::cmp::Ordering::Greater => r -= 1,
            }
        }
    }

    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return [].into()
        }
        nums.sort_unstable();
        let nums = nums;
        let mut res = Vec::new();
        for i in 0..nums.len() - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            if nums[i] + nums[i+1] + nums[i+2] > 0 {
                // no-possible answer anymore
                break;
            }

            let slice: &[i32] = &nums[i+1..];
            let target = -nums[i];
            Self::two_sum_on_sorted(slice, target, &mut res);
        }
        res
    }
}

