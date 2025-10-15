use super::Solution;

#[derive(PartialEq, Eq)]
enum State {
    First(usize),
    Second(usize),
    SecondEx(usize),    // the whole sequence is increasing
}


impl Solution {
    pub fn has_increasing_subarrays(nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize;
        if nums.is_empty() {
            return false;
        }
        use State::*;
        let mut state = State::First(1);
        let mut last = nums[0];
        for num in nums.into_iter().skip(1) {
            match (state, num) {
                (SecondEx(cnt), _) if cnt == k => {
                    return true;
                }
                (SecondEx(cnt), val) if val > last => {
                    state = SecondEx(cnt + 1);
                    last = val;
                }
                (SecondEx(_), val) => {
                    // match failed
                    // but at here we have k + cnt increasing numbers in a row
                    state = Second(1);
                    last = val;
                }
                (Second(cnt), _) if cnt == k => {
                    // Finished! Cool!
                    return true;
                }
                (Second(cnt), val) if val > last => {
                    state = Second(cnt + 1);
                    last = val;
                }
                (Second(_), val) => {
                    // match failed
                    state = First(1);
                    last = val;
                }
                (First(count), val) if count == k => {
                    state = if val > last {
                        SecondEx(1)
                    } else {
                        Second(1)
                    };
                    last = val;
                }
                (First(count), val) if val > last => {
                    state = First(count + 1);
                    last = val;
                }
                (First(_), val) => {
                    state = First(1);
                    last = val;
                }
            }
        }

        state == Second(k) || state == SecondEx(k)
    }
}
#[cfg(test)]
mod tests {

    #[test]
    fn test_has_increasing_subarrays_true() {
        let nums: Vec<i32> = vec![2, 5, 7, 8, 9, 2, 3, 4, 3, 1];
        let k = 3;
        assert_eq!(super::Solution::has_increasing_subarrays(nums, k), true);
    }

    #[test]
    fn test_finish_at_end() {
        let nums = vec![-15, 19];
        assert!(super::Solution::has_increasing_subarrays(nums, 1));
    }
}
