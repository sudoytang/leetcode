use super::Solution;

impl Solution {
    unsafe fn swap_4(v1: *mut i32, v2: *mut i32, v3: *mut i32, v4: *mut i32) {
        unsafe {
            let temp = *v1;
            *v1 = *v4;
            *v4 = *v3;
            *v3 = *v2;
            *v2 = temp;
        }
    }

    fn swap_pos(matrix: &mut Vec<Vec<i32>>, pos: (usize, usize)) {
        let n = matrix.len();
        let (y1, x1) = pos;
        let (y2, x2) = (x1, n - 1 - y1);
        let (y3, x3) = (x2, n - 1 - y2);
        let (y4, x4) = (x3, n - 1 - y3);
        unsafe {
            Self::swap_4(&mut matrix[y1][x1], &mut matrix[y2][x2], &mut matrix[y3][x3], &mut matrix[y4][x4]);
        }
    }

    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for y in 0..n/2 {
            for x in y..n-y-1 {
                Self::swap_pos(matrix, (y, x));
            }
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_basic_3x3() {
        let mut res = vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]];
        super::Solution::rotate(&mut res);
        let reference = vec![vec![7,4,1],vec![8,5,2],vec![9,6,3]];
        assert_eq!(res, reference);
    }

    #[test]
    fn test_basic_4x4() {
        let mut res = [[5,1,9,11],[2,4,8,10],[13,3,6,7],[15,14,12,16]]
            .into_iter().map(|v|v.into()).collect();
        super::Solution::rotate(&mut res);
        let reference = [[15,13,2,5],[14,3,4,1],[12,6,8,9],[16,7,10,11]];
        assert_eq!(res, reference);
    }
}