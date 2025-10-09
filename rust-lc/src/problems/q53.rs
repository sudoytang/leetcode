#[allow(unused)]
struct Solution;
#[allow(unused)]
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        // sub problem R[i]: max_sub_array of nums[0..=i], and the index i is selected
        // if R[i-1] >= 0, then R[i] = R[i-1] + nums[i] (we have to choose nums[i])
        // if R[i-1] < 0, then R[i] = nums[i], because we can start from here
        let mut r = vec![0; nums.len()];
        for i in 0..r.len() {
            if i == 0 {
                r[i] = nums[i];
            } else {
                r[i] = nums[i].max(r[i-1] + nums[i]);
            }
        }

        r.into_iter().max().unwrap()
    }
}