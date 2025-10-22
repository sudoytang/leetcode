use super::Solution;

impl Solution {

    /// find the first index that the element is NOT LESS than `target`
    /// or `nums.len()`
    pub fn lower_bound(nums: &[i32], target: i32) -> usize {
        let mut first = 0;
        let mut count = nums.len() - first;

        while count > 0 {
            let step = count / 2;
            let it = first + step;

            if nums[it] < target {
                first = it + 1;
                count -= step + 1;
            } else {
                count = step;
            }
        }

        first

    }
    /// find the index that the element is GREATER than `target`
    /// or `nums.len()`
    pub fn upper_bound(nums: &[i32], target: i32) -> usize {
        let mut first = 0;
        let mut count = nums.len() - first;

        while count > 0 {
            let step = count / 2;
            let it = first + step;

            if !(target < nums[it]) {
                first = it + 1;
                count -= step + 1;
            } else {
                count = step;
            }
        }

        first
    }

    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let lower_bound = Self::lower_bound(&nums, target);
        let upper_bound = Self::upper_bound(&nums, target);
        if lower_bound == upper_bound {
            // ok, nothing satisfies
            vec![-1, -1]
        } else {
            vec![lower_bound as i32, upper_bound as i32 - 1]
        }
    }
}