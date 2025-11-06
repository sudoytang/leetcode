
#[allow(unused)]
pub struct Solution;
#[allow(unused)]
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        if nums.len() < 2 {
            return;
        }
        let mut pivot_idx_add1 = nums.len() - 1;
        while pivot_idx_add1 > 0 && nums[pivot_idx_add1 - 1] >= nums[pivot_idx_add1] {
            pivot_idx_add1 -= 1;
        }
        if pivot_idx_add1 == 0 {
            nums.reverse();
            return;
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
            },
            Err(idx) => {
                idx
            }
        } + pivot_idx_add1;
        nums.swap(pivot_idx_add1 - 1, upper_bound);
    }
}