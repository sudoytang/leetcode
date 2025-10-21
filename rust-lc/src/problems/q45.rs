use super::Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut max_reachable = nums[0] as usize;
        let mut last_reachable = max_reachable;
        if nums.len() == 1 {
            return 0;
        }

        let mut steps = 1;
        for i in 0..nums.len() - 1 {
            max_reachable = max_reachable.max(i + nums[i] as usize);
            if i == last_reachable {
                last_reachable = max_reachable;
                steps += 1;
            }

        }
        steps
    }
}


#[cfg(test)]
mod test {
    use crate::problems::Solution;

    #[test]
    fn jump_21() {
        let res = Solution::jump([2, 1].into());
        assert_eq!(1, res);
    }

    #[test]
    fn jump_23114() {
        let res = Solution::jump([2, 3, 1, 1, 4].into());
        assert_eq!(2, res);
    }

    #[test]
    fn jump_123() {
        let res = Solution::jump([1, 2, 3].into());
        assert_eq!(2, res);
    }
}