use std::collections::VecDeque;

#[allow(unused)]
pub struct Solution;
#[allow(unused)]
impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut q = VecDeque::new();
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == 2 /* rotten */ {
                    q.push_back((i, j, 0));
                }
            }
        }
        let mut max_tick = 0;
        while let Some((x, y, tick)) = q.pop_front() {
            max_tick = max_tick.max(tick);
            if x > 0 && grid[x - 1][y] == 1 {
                grid[x - 1][y] = 2;
                q.push_back((x - 1, y, tick + 1));
            }
            if y > 0 && grid[x][y - 1] == 1 {
                grid[x][y - 1] = 2;
                q.push_back((x, y - 1, tick + 1));
            }
            if x < grid.len() - 1 && grid[x + 1][y] == 1 {
                grid[x + 1][y] = 2;
                q.push_back((x + 1, y, tick + 1));
            }
            if y < grid[0].len() - 1 && grid[x][y + 1] == 1 {
                grid[x][y + 1] = 2;
                q.push_back((x, y + 1, tick + 1));
            }
        }
        if grid.iter().flat_map(|v| v.iter()).any(|&o| o == 1) {
            -1
        } else {
            max_tick
        }
    }
}