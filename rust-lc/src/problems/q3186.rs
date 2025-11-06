use std::collections::BTreeMap;

// almost the same as q740;


#[cfg(feature = "local")]
#[allow(unused)]
pub struct Solution;

#[allow(unused)]
impl Solution {
    pub fn maximum_total_damage(power: Vec<i32>) -> i64 {

        let map = power.into_iter().fold(BTreeMap::new(), |mut map, num| {
            *map.entry(num as i64).or_insert(0) += 1i64;
            map
        });

        let mut last = i64::MIN;
        let mut v1 = 0;
        let mut v2 = 0;
        let mut v3 = 0;
        // current = max(v1, v2 + count * i)
        for (i, count) in map {
            if last < i - 2 {
                v3 = v1;
                v2 = v1;
            } else if last == i - 2 {
                v3 = v2;
                v2 = v1;
            }
            let cur = v1.max(v2).max(v3 + i * count);
            last = i;
            v3 = v2;
            v2 = v1;
            v1 = cur;
        }
        v1
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_q3186_basic() {
        let res = Solution::maximum_total_damage([1, 1, 3, 4].into());
        assert_eq!(res, 6);
    }
}