#[cfg(feature = "local")]
#[allow(unused)]
pub struct Solution;

#[allow(unused)]

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return -1;
        }
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

        // now first is the smallest number in the whole array

        let (mut l, r) = match target.cmp(&pivot) {
            std::cmp::Ordering::Less => {
                (first, nums.len())
            }
            std::cmp::Ordering::Equal => {
                return nums.len() as i32 - 1;
            }
            std::cmp::Ordering::Greater => {
                (0, first)
            }
        };

        let mut count = r - l;
        while count > 0 {
            let step = count / 2;
            let it = l + step;
            if nums[it] < target {
                l = it + 1;
                count -= step + 1;
            } else {
                count = step;
            }
        }

        if l == nums.len() {
            -1
        } else if nums[l] != target {
            -1
        } else {
            l as i32
        }

    }
}