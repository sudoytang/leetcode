#[cfg(feature = "local")]
#[allow(unused)]
pub struct Solution;

#[allow(unused)]

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {

        let mut i = 0;
        let mut max_reachable_idx = nums[i] as usize;

        while i <= max_reachable_idx {
            max_reachable_idx = max_reachable_idx.max(i + nums[i] as usize);
            if max_reachable_idx >= nums.len() - 1 {
                return true;
            }
            i += 1;
        }



        max_reachable_idx >= nums.len() - 1
    }
}


#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_q55_ok() {
        let res = Solution::can_jump([2,3,1,1,4].into());
        assert!(res)
    }

    #[test]
    fn test_q55_ok_123() {
        let res = Solution::can_jump([1, 2, 3].into());
        assert!(res)
    }
}