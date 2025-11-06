#[allow(unused)]
pub struct Solution;
#[allow(unused)]
impl Solution {

    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        (0..num_rows as usize).fold(Vec::new(), |mut acc, i| {
            let mut new_row = vec![0; i + 1];

            match acc.last() {
                Some(row) => {
                    for i in 0..=row.len() {
                        match i {
                            0 => new_row[i] = 1,
                            i if i == row.len() => new_row[i] = 1,
                            i => new_row[i] = row[i-1] + row[i],
                        }

                    }
                }
                None => new_row[0] = 1,
            }
            acc.push(new_row);
            acc
        })
    }
}