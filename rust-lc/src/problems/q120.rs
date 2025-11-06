struct Memo {
    memo: Vec<Vec<i32>>,
}

impl Memo {
    pub fn new(size: usize) -> Self {
        let mut memo = Vec::new();
        for i in 0..size {
            memo.push(vec![i32::MAX; i + 1]);
        }
        Self { memo }
    }

    fn get_left_parent(pos: (usize, usize)) -> Option<(usize, usize)> {
        match pos {
            (0, _) => None,
            (_, 0) => None,
            (r, c) => Some((r - 1, c - 1))
        }
    }

    fn get_right_parent(pos: (usize, usize)) -> Option<(usize, usize)> {
        match pos {
            (0, _) => None,
            (r, c) if r == c => None,
            (r, c) => Some((r - 1, c))
        }
    }

    fn min_total_at(&mut self, triangle: &Vec<Vec<i32>>, pos: (usize, usize)) -> i32 {
        let (r, c) = pos;
        if self.memo[r][c] != i32::MAX {
            return self.memo[r][c];
        }
        let res = match (Self::get_left_parent(pos), Self::get_right_parent(pos)) {
            (None, None) => triangle[r][c],
            (Some(lp), None) => triangle[r][c] + self.min_total_at(triangle, lp),
            (None, Some(rp)) => triangle[r][c] + self.min_total_at(triangle, rp),
            (Some(lp), Some(rp)) => {
                triangle[r][c] +
                i32::min(self.min_total_at(triangle, lp), self.min_total_at(triangle, rp))
            }
        };
        self.memo[r][c] = res;
        res
    }
}


#[allow(unused)]
pub struct Solution;
#[allow(unused)]

impl Solution {


    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let r = triangle.len() - 1;
        let last = triangle.last().unwrap();
        let mut min = i32::MAX;
        let mut memo = Memo::new(triangle.len());
        for c in 0..last.len() {
            min = min.min(memo.min_total_at(&triangle, (r, c)));
        }
        min
    }
}