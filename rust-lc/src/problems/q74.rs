use std::ops::Index;


#[allow(unused)]
pub struct Solution;

struct FlattenMat<'v> {
    vec: &'v Vec<Vec<i32>>,
}

impl<'v> FlattenMat<'v> {
    fn new(vec: &'v Vec<Vec<i32>>) -> Self {
        Self {
            vec,
        }
    }

    fn unravel(&self, i: usize) -> (usize, usize) {
        let cols = self.vec[0].len();
        let row = i / cols;
        let col = i % cols;
        (row, col)
    }

    fn len(&self) -> usize {
        self.vec.len() * self.vec[0].len()
    }
}

impl<'v> Index<usize> for FlattenMat<'v> {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        let (row, col) = self.unravel(index);
        &self.vec[row][col]
    }
}

#[allow(unused)]
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let flatten = FlattenMat::new(&matrix);
        let mut l = 0;
        let mut r = flatten.len();
        while l < r {
            if l + 1 == r {
                // one left
                if flatten[l] == target {
                    return true;
                } else {
                    return false;
                }
            }
            let mid = (l + r) / 2;
            match flatten[mid].cmp(&target) {
                std::cmp::Ordering::Less => l = mid + 1,
                std::cmp::Ordering::Equal => return true,
                std::cmp::Ordering::Greater => r = mid,
            }
        }
        false
    }
}