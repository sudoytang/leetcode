#[cfg(feature = "local")]
#[allow(unused)]
pub struct Solution;

#[allow(unused)]
impl Solution {
    pub fn fib(n: i32) -> i32 {
        let mut memo : Memo = Default::default();
        memo.fib(n as usize)
    }
}

#[derive(Default)]
struct Memo {
    memo: Vec<Option<i32>>
}

impl Memo {
    pub fn fib(&mut self, n: usize) -> i32 {
        if self.memo.len() <= n {
            self.memo.resize_with(n + 1, || None);
        }
        if let Some(val) = self.memo[n] {
            return val;
        } else if n == 0 {
            self.memo[n] = Some(0);
        } else if n == 1 {
            self.memo[n] = Some(1);
        } else {
            self.memo[n] = Some(self.fib(n - 1) + self.fib(n - 2));
        }

        self.memo[n].unwrap()
    }
}


