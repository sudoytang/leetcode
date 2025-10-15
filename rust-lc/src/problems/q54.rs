use super::Solution;



impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        if matrix.len() == 0 || matrix[0].len() == 0 {
            return vec![];
        }


        let rows = matrix.len() as isize;
        let cols = matrix[0].len() as isize;
        let mut res = Vec::new();

        let mut lb = 0;
        let mut rb = cols - 1;
        let mut ub = 0;
        let mut db = rows - 1;

        while lb <= rb && ub <= db {
            for x in lb..=rb {
                res.push(matrix[ub as usize][x as usize]);
            }
            for y in ub+1..=db {
                res.push(matrix[y as usize][rb as usize]);
            }
            if lb < rb && ub < db {
                for x in (lb+1..=rb-1).rev() {
                    res.push(matrix[db as usize][x as usize]);
                }
                for y in (ub+1..=db).rev() {
                    res.push(matrix[y as usize][lb as usize]);
                }
            }
            lb += 1;
            rb -= 1;
            ub += 1;
            db -= 1;
        }

        res
    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn test_single() {
        let matrix = vec![vec![1]];
        assert_eq!(super::Solution::spiral_order(matrix), vec![1]);
    }

}
