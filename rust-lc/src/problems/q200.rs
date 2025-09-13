#[allow(unused)]
pub struct Solution;
#[allow(unused)]
impl Solution {
    pub fn infect_from(grid: &mut Vec<Vec<char>>, i: usize, j: usize) -> bool {
        // '0' -> water
        // '1' -> land
        // ext: '2' -> visited land
        if i >= grid.len() {
            false
        } else if j >= grid[0].len() {
            false
        } else if matches!(grid[i][j], '0' | '2') {
            false
        } else {
            grid[i][j] = '2';
            if i > 0 {
                Self::infect_from(grid, i - 1, j);
            }
            if j > 0 {
                Self::infect_from(grid, i, j - 1);
            }
            Self::infect_from(grid, i + 1, j);
            Self::infect_from(grid, i, j + 1);
            true
        }

    }
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let cartisan_prod = (0..m).flat_map(|i| (0..n).map(move |j| (i, j)));
        cartisan_prod
            .map(|(i, j)| Self::infect_from(&mut grid, i, j))
            .filter(|&p| p)
            .count() as i32
    }
}