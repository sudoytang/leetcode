#[allow(unused)]
struct Solution;
#[allow(unused)]

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut memo = Memo::new();
        memo.climb_stairs(n as usize)
    }
}

struct Memo {
    memo: Vec<Option<i32>>
}

impl Memo {
    pub fn new() -> Self {
        Self {
            memo: Vec::new()
        }
    }

    pub fn climb_stairs(&mut self, n: usize) -> i32 {
        if n >= self.memo.len() {
            self.memo.resize_with(n + 1, || None);
        }
        match self.memo[n] {
            Some(val) => val,
            None if n == 0 => {
                self.memo[n] = Some(1);
                1
            }
            None => {
                // calculate how many ways I can use to climb to step n.
                // which is the sum of ways that I can use to climb to n - 1
                // and ways I can use to climb to n - 2
                let mut res = self.climb_stairs(n - 1);
                if n >= 2 {
                    res += self.climb_stairs(n - 2);
                }
                self.memo[n] = Some(res);
                res
            }
        }
    }
}