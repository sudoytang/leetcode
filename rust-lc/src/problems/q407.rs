#[allow(unused)]
struct Solution;
#[allow(unused)]
impl Solution {
    pub fn trap_rain_water(mut height_map: Vec<Vec<i32>>) -> i32 {
        use std::collections::BinaryHeap;
        let rows = height_map.len();
        let cols = height_map[0].len();
        let mut bh = BinaryHeap::new();
        for (i, r) in height_map.iter_mut().enumerate() {
            for (j, c) in r.iter_mut().enumerate() {
                if i == 0 || i == rows - 1 || j == 0 || j == cols - 1 {
                    bh.push((-*c, i, j));
                    *c = -1;
                }
            }
        }
        let mut acc = 0;
        while let Some((min_h, i, j)) = bh.pop() {
            let min_h = -min_h;
            for (x, y) in [(i.wrapping_sub(1), j), (i + 1, j), (i, j.wrapping_sub(1)), (i, j + 1)] {
                if x >= rows || y >= cols || height_map[x][y] < 0 {
                    continue;
                }
                acc += 0.max(min_h - height_map[x][y]);
                bh.push((-(min_h.max(height_map[x][y])), x, y));
                height_map[x][y] = -1;
            }
        }

        acc

    }
}