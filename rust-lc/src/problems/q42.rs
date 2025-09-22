use std::fmt::Debug;



fn iter_diag<T>(iter: impl Iterator<Item = T>)
where T: Debug,
{
    println!("{:?}", iter.collect::<Vec<_>>());
}


#[allow(unused)]
struct Solution;
#[allow(unused)]
impl Solution {
    fn trap_dp(height: Vec<i32>) -> i32 {
        let mut height_lr: Vec<(i32, i32)> = Vec::new();
        let left = height.iter().scan(0, |state, &i| {
            *state = i32::max(i, *state);
            Some(*state)
        });
        iter_diag(left.clone());
        let right = height.iter().rev().scan(0, |state, &i| {
            *state = i32::max(i, *state);
            Some(*state)
        }).collect::<Vec<_>>().into_iter().rev();
        iter_diag(right.clone());
        left.zip(right).zip(height.iter().copied()).map(|((l, r), h)| i32::min(l, r) - h).sum()
    }

    fn trap_stk(height: Vec<i32>) -> i32 {
        let mut stk: Vec<(i32, usize)> = Vec::new();
        let mut res = 0;
        for (i, elem) in height.iter().copied().enumerate() {
            while stk.last().is_some_and(|&(h, idx)| h < elem) {
                let (h, idx) = stk.pop().unwrap();
                if stk.last().is_none() {
                    break;
                }
                let &(lh, lidx) = stk.last().unwrap();
                let xdiff = i - 1 - stk.last().unwrap().1;
                let height = i32::min(elem, lh) - h;
                res += height * xdiff as i32;
            }
            stk.push((elem, i));
        }
        res
    }

    pub fn trap(height: Vec<i32>) -> i32 {
        Self::trap_stk(height)
    }
}
