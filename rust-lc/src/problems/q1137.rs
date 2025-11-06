#[cfg(feature = "local")]
#[allow(unused)]
pub struct Solution;

#[allow(unused)]
impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        match n {
            0 => return 0,
            1 => return 1,
            2 => return 1,
            n if n < 0 => unreachable!(),
            _ => {}
        }

        let mut v3 = 0;
        let mut v2 = 1;
        let mut v1 = 1;
        for _ in 3..=n {
            let cur = v3 + v2 + v1;
            v3 = v2;
            v2 = v1;
            v1 = cur;
        }

        v1
    }
}