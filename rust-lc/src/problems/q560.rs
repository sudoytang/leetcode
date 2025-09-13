use std::collections::HashMap;

#[allow(unused)]
struct Solution;
#[allow(unused)]
impl Solution {
    pub fn subarray_sum(mut nums: Vec<i32>, k: i32) -> i32 {
        let mut acc = 0;
        for num in nums.iter_mut() {
            acc += *num;
            *num = acc;
        }
        let mut hs: HashMap<i32, i32> = HashMap::new();
        let mut res = 0;
        for num in nums {
            // find num - k in hm
            if num == k {
                res += 1;
            }
            hs.entry(num - k).and_modify(|v| res += *v);
            *hs.entry(num).or_default() += 1;
        }
        res
    }
}