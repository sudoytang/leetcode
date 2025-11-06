#[cfg(feature = "local")]
#[allow(unused)]
pub struct Solution;

#[allow(unused)]

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        // the elements in monotonic_stk should have monotonic decreasing order
        let mut monotonic_stk: Vec<(usize, i32)> = Vec::new();
        let mut answer = vec![0; temperatures.len()];
        // so that when a higher temperature appears
        // we can get the answer of the top element of the stack and pop it.
        for (i, temp) in temperatures.into_iter().enumerate() {
            while let Some((day, _)) = monotonic_stk.pop_if(|&mut (_, low)| low < temp){
                answer[day] = (i - day) as i32;
            }
            monotonic_stk.push((i, temp));

        }

        answer

    }
}