#[allow(unused)]
struct Solution;
#[allow(unused)]
impl Solution {
    pub fn product_except_self(mut nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();

        let presuf: Vec<(i32, i32)> = nums.iter().scan(1, |acc, i| {
            *acc *= *i;
            return Some(*acc);
        }).zip(nums.iter().rev().scan(1, |acc, i| {
            *acc *= *i;
            return Some(*acc);
        }).collect::<Vec<_>>().into_iter().rev()).collect();
        print_iter(presuf.clone());
        nums.iter_mut().enumerate().for_each(|(i, v)| {
            *v = if i == 0 {
                1 * presuf[i + 1].1
            } else if i == len - 1 {
                presuf[i - 1].0 * 1
            } else {
                presuf[i - 1].0 * presuf[i + 1].1
            }
        });
        nums

    }
}

fn print_iter<I>(iter: I)
where
    I: IntoIterator,
    I::Item: std::fmt::Debug,
{
    for item in iter {
        print!("{:?} ", item);
    }
    println!();
}