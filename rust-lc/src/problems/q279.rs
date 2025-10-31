use super::Solution;


struct Memo {
    r: Vec<Vec<i32>>,
}

impl Memo {
    fn new(n: usize) -> Self {
        let rows = n.isqrt() + 1;
        let mut r = vec![vec![-1; n + 1]; rows];
        for (i, elem) in r[0].iter_mut().enumerate() {
            if i == 0 {
                *elem = 0;
            } else {
                *elem = i32::MAX;
                // we cannot get any other number with only zeros.
            }
        }
        for v in r.iter_mut() {
            v[0] = 0;
            // we don't need anything to get zero
        }
        Self {
            r
        }
    }

    fn calc(&mut self, i: usize, j: usize) -> i32 {
        if self.r[i][j] < 0 {
            // r[i, j] the min count of numbers to form j using (0..i).map(|i| i * i)
            // r[i, j] = r[i - 1, j] (not choosing i)
            // r[i, j] = r[i, j - i^2] + 1 (choosing i), where j - i^2 >= 0
            if j >= i * i {
                self.r[i][j] = self.calc(i-1, j).min(self.calc(i, j - i*i) + 1);
            } else {
                self.r[i][j] = self.calc(i-1, j);
            }
        }
        self.r[i][j]

    }
}


impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let n = n as usize;

        let mut memo = Memo::new(n);
        memo.calc(n.isqrt(), n)
    }
}