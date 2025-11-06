#[allow(unused)]
pub struct Solution;
#[allow(unused)]
impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_unstable_by_key(|v| v[0]);

        intervals.into_iter().fold(Vec::new(), |mut acc, rng| {
            if let Some(last_rng) = acc.last_mut() {
                match last_rng[1].cmp(&rng[0]) {
                    std::cmp::Ordering::Less /* overlap */ => acc.push(rng),
                    std::cmp::Ordering::Equal => last_rng[1] = last_rng[1].max(rng[1]),
                    std::cmp::Ordering::Greater => last_rng[1] = last_rng[1].max(rng[1]),
                }
            } else {
                acc.push(rng);
            }
            acc
        })
    }
}