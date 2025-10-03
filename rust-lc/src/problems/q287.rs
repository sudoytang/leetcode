#[allow(unused)]
pub struct Solution;
#[allow(unused)]

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut fast = VecLLIter::new(&nums);
        let mut slow = VecLLIter::new(&nums);
        loop {
            fast.next();
            let fast_val = fast.next();
            let slow_val = slow.next();
            if fast_val == slow_val {
                break;
            }
        }
        let mut it = VecLLIter::new(&nums);
        loop {
            let it_val = it.next();
            let slow_val = slow.next();
            if slow_val == it_val {
                return slow_val.unwrap()
            }
        }
    }
}


pub struct VecLLIter<'a> {
    borrow: &'a Vec<i32>,
    current: usize,
}

impl<'a> VecLLIter<'a> {
    pub fn new(nums: &'a Vec<i32>) -> Self {
        Self {
            borrow: nums,
            current: 0,
        }
    }
    pub fn current_idx(&self) -> usize {
        self.current
    }
    pub fn current_val(&self) -> Option<&i32> {
        self.borrow.get(self.current_idx())
    }
}

impl<'a> Iterator for VecLLIter<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.borrow.get(self.current).copied();
        match res {
            Some(n) => {
                self.current = n as usize;
                Some(n)
            }
            None => None
        }
    }
}