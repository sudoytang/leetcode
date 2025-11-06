#[cfg(feature = "local")]
#[allow(unused)]
pub struct Solution;

#[allow(unused)]

struct State<'a> {
    matref: &'a mut Vec<Vec<i32>>,
    is_col_0_has_0: bool,
    is_row_0_has_0: bool,
}

impl<'a> State<'a> {
    fn from(matrix: &'a mut Vec<Vec<i32>>) -> Self {
        Self {
            matref: matrix,
            is_col_0_has_0: false,
            is_row_0_has_0: false,
        }
    }

    fn is_col_j_has_0(&self, j: usize) -> bool {
        if j == 0 {
            self.is_col_0_has_0
        } else {
            self.matref[0][j] == 0
        }
    }

    fn is_row_i_has_0(&self, i: usize) -> bool {
        if i == 0 {
            self.is_row_0_has_0
        } else {
            self.matref[i][0] == 0
        }
    }

    fn set_row_i_has_0(&mut self, i: usize) {
        if i == 0 {
            self.is_row_0_has_0 = true;
        } else {
            self.matref[i][0] = 0;
        }
    }
    fn set_col_j_has_0(&mut self, j: usize) {
        if j == 0 {
            self.is_col_0_has_0 = true;
        } else {
            self.matref[0][j] = 0;
        }
    }
    fn inspect(&mut self) {

        for i in 0..self.matref.len() {
            for j in 0..self.matref[0].len() {
                if self.matref[i][j] == 0 {
                    self.set_row_i_has_0(i);
                    self.set_col_j_has_0(j);
                }
            }
        }
        for i in 1..self.matref.len() {
            for j in 1..self.matref[0].len() {
                if self.is_row_i_has_0(i) || self.is_col_j_has_0(j) {
                    self.matref[i][j] = 0;
                }
            }
        }
        if self.is_row_0_has_0 {
            for j in 0..self.matref[0].len() {
                self.matref[0][j] = 0;
            }
        }
        if self.is_col_0_has_0 {
            for i in 0..self.matref.len() {
                self.matref[i][0] = 0;
            }
        }
    }
}


impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut state = State::from(matrix);
        state.inspect();
    }
}