use super::Solution;

impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let weeks = n / 7;
        // in the first week Hercy would deposit 1 + 2 + 3 + 4 + 5 + 6 + 7 = 28
        // in the i-th week Hercy would deposit 28 + 7 * i (at the first week i = 0)
        let remain_days = n % 7;
        (28 + (28 + (weeks - 1) * 7)) * weeks / 2 +
        ((weeks + 1) + (weeks + 1 + remain_days - 1)) * remain_days / 2

    }
}

#[cfg(test)]
mod test {
    use crate::problems::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::total_money(4), 10);
    }
    #[test]
    fn test_2() {
        assert_eq!(Solution::total_money(10), 37);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::total_money(20), 96);
    }
}