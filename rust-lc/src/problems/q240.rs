use super::Solution;
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut y = 0;
        let mut x = cols - 1;
        while y < rows && x < cols {
            match matrix[y][x].cmp(&target) {
                std::cmp::Ordering::Less => y += 1,
                std::cmp::Ordering::Equal => return true,
                std::cmp::Ordering::Greater => x = x.wrapping_sub(1),
            }
        }

        false
    }
}