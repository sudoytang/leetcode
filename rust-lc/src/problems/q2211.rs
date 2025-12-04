#[cfg(feature = "local")]
#[allow(unused)]
pub struct Solution;

#[cfg(feature = "local")]
#[allow(unused)]
use crate::ListNode;

#[cfg(feature = "local")]
#[allow(unused)]
use crate::TreeNode;

impl Solution {
    pub fn count_collisions(directions: String) -> i32 {
        let mut directions = directions.into_bytes();
        // let mut finished = directions.iter().copied().all(|d| d != b'L' && d != b'R');
        // enumerating gaps
        let mut count = 0;
        // while !finished {
        for i in 0..=directions.len() {
            if i == 0 {
                // leftmost one
                if directions[i] == b'L' {
                    directions[i] = b'E';
                }
            } else if i == directions.len() {
                // rightmost one
                if directions[i-1] == b'R' {
                    directions[i-1] = b'E';
                }
            } else {
                match (directions[i-1], directions[i]) {
                    (b'E', b'L') => directions[i] = b'E',
                    (b'E', _) => {}

                    (b'L', _) => {}
                    (b'S', b'L') => {count += 1; directions[i] = b'S'}
                    (b'S', _) => {},
                    (b'R', b'L') => {count += 2; directions[i-1] = b'S'; directions[i] = b'S';}
                    (b'R', b'S') => {count += 1; directions[i-1] = b'S'}
                    (b'R', b'E') => directions[i-1] = b'E',
                    (b'R', _) => {}
                    _ => {}
                }
            }
        }
        for i in (0..=directions.len()).rev() {
            if i == 0 {
                // leftmost one
                if directions[i] == b'L' {
                    directions[i] = b'E';
                }
            } else if i == directions.len() {
                // rightmost one
                if directions[i-1] == b'R' {
                    directions[i-1] = b'E';
                }
            } else {
                match (directions[i-1], directions[i]) {
                    (b'E', b'L') => directions[i] = b'E',
                    (b'E', _) => {}

                    (b'L', _) => {}
                    (b'S', b'L') => {count += 1; directions[i] = b'S'}
                    (b'S', _) => {},
                    (b'R', b'L') => {count += 2; directions[i-1] = b'S'; directions[i] = b'S';}
                    (b'R', b'S') => {count += 1; directions[i-1] = b'S'}
                    (b'R', b'E') => directions[i-1] = b'E',
                    (b'R', _) => {}
                    _ => {}
                }
            }
        }
        // }

        count
    }
}