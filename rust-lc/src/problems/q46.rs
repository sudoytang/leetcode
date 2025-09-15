#[allow(unused)]
struct Solution;
#[allow(unused)]
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) -> bool {
        if nums.len() < 2 {
            return true;
        }
        let mut pivot_idx_add1 = nums.len() - 1;
        while pivot_idx_add1 > 0 && nums[pivot_idx_add1 - 1] >= nums[pivot_idx_add1] {
            pivot_idx_add1 -= 1;
        }
        if pivot_idx_add1 == 0 {
            nums.reverse();
            return true;
        }
        let pivot = nums[pivot_idx_add1 - 1];
        let slice = &mut nums[pivot_idx_add1..];
        slice.reverse();

        let upper_bound = match slice.binary_search(&pivot) {
            Ok(mut idx) => {
                while slice[idx] == pivot {
                    idx += 1;
                }
                idx
            }
            Err(idx) => idx
        };
        nums.swap(pivot_idx_add1 - 1, upper_bound);
        return false;
    }

    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut nums = nums;
        nums.sort_unstable();
        while !Self::next_permutation(&mut nums) {
            res.push(nums.clone());
        }
        res
    }
}