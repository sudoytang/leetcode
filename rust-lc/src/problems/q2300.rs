#[allow(unused)]
pub struct Solution;
#[allow(unused)]
impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
        let mut potions = potions;
        potions.sort_unstable();

        spells.into_iter().map(|spell| {
            // the binary search perdicate is spell * potion >= success
            (potions.len() - potions.partition_point(|&potion| {
                (potion as i64 * spell as i64) < success
            })) as i32
        }).collect()
    }
}