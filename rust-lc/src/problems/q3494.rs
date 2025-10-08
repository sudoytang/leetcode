
#[allow(unused)]
pub struct Solution;
#[allow(unused)]

impl Solution {
    pub fn min_time(skill: Vec<i32>, mana: Vec<i32>) -> i64 {
        let mut available_time = vec![0i64; skill.len()];
        for potion in mana {
            let potion = potion as i64;
            for i in 0..skill.len() {
                // forward propagate (if the former wizard is bottleneck)
                available_time[i] = available_time[i].max(*available_time.get(i.wrapping_sub(1)).unwrap_or(&0)) + potion * skill[i] as i64;
            }
            for i in (0..skill.len()).rev() {
                // backward propagate if the latter wizard is bottleneck
                available_time[i] = available_time[i].max(*available_time.get(i + 1).unwrap_or(&available_time[i]) - potion * *skill.get(i + 1).unwrap_or(&0) as i64)
            }
        }
        *available_time.last().unwrap()
    }
}