#[allow(unused)]
pub struct Solution;
#[allow(unused)]
impl Solution {
    pub fn num_water_bottles(mut num_bottles: i32, num_exchange: i32) -> i32 {
        let mut sum = num_bottles;
        while num_bottles >= num_exchange {
            let remainder = num_bottles % num_exchange;  // remaining empty bottles
            num_bottles /= num_exchange;
            sum += num_bottles;
            num_bottles += remainder;
        }

        sum
    }
}