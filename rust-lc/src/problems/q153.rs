use super::Solution;
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let pivot = nums[nums.len() - 1];
        // find the first element that less than pivot
        // this is the smallest number in the whole array

        let mut first = 0;
        let mut count = (nums.len() - 1 - first) as isize;
        while count > 0 {
            let step = count / 2;
            let it = first + step as usize;

            if nums[it] > pivot {
                first = it + 1;
                count -= step + 1;
            } else {
                count = step;
            }
        }
        nums[first]
    }
}