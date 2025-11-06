#[cfg(feature = "local")]
#[allow(unused)]
pub struct Solution;

#[allow(unused)]

impl Solution {
    pub fn search_insert_impl(nums: &[i32], target: i32) -> usize {
        assert!(nums.len() > 0);

        if nums.len() == 1 {
            let val = nums[0];
            match target.cmp(&val) {
                std::cmp::Ordering::Less => 0,
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Greater => 1,
            }
        } else {
            // at here the size of the array is no less than 2
            let mid = nums.len() / 2;
            let val = nums[mid];
            match target.cmp(&val) {
                std::cmp::Ordering::Less => Self::search_insert_impl(&nums[..mid], target),
                std::cmp::Ordering::Equal => mid,
                std::cmp::Ordering::Greater => {
                    if mid == nums.len() - 1 {
                        nums.len()
                    } else {
                        mid + 1 + Self::search_insert_impl(&nums[mid+1..], target)
                    }
                }
            }
        }
    }
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        Self::search_insert_impl(&nums, target) as i32
    }
}