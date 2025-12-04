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
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        enum State {
            Idle,
            Got1,
        }
        enum Char {
            C0,
            C10,
            C11
        }
        let mut state = State::Idle;
        let mut last: Option<Char> = None;
        use Char::*;
        use State::*;
        for bit in bits {
            (state, last) = match (state, bit, last) {
                (Idle, 0, _) => (Idle, Some(C0)),
                (Idle, 1, l) => (Got1, l),
                (Got1, 0, _) => (Idle, Some(C10)),
                (Got1, 1, _) => (Idle, Some(C11)),
                _ => unreachable!()
            }
        };


        last.map(|ch| matches!(ch, Char::C0)).unwrap_or(false)

    }
}