#[cfg(feature = "local")]
#[allow(unused)]
pub struct Solution;

#[cfg(feature = "local")]
#[allow(unused)]
use crate::ListNode;

#[cfg(feature = "local")]
#[allow(unused)]
use crate::TreeNode;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut lens = vec![0usize; nums.len() + 1];
        // lens[i] = length of lis of nums[0..i]
        // result = lens[nums.len()]
        // lens[i] = lens[j] + 1 if nums[i - 1] > j
        for i in 1..lens.len() {
            for j in 0..i {
                if nums[i - 1] > nums[j] {
                    lens[i] = lens[i].max(lens[j + 1])
                }
            }
            lens[i] += 1;
        }
        lens.into_iter().max().unwrap() as i32
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_q300() {
        let input = vec![1,3,6,7,9,4,10,5,6];
        assert_eq!(super::Solution::length_of_lis(input), 6);
    }
}