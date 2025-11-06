#[allow(unused)]
pub struct Solution;
#[allow(unused)]
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stk = Vec::new();
        for b in s.as_bytes().iter().copied() {
            match b {
                b'(' | b'{' | b'[' => stk.push(b),
                b')' => {
                    let Some(b'(') = stk.pop() else {
                        return false;
                    };
                }
                b']' => {
                    let Some(b'[') = stk.pop() else {
                        return false;
                    };
                }
                b'}' => {
                    let Some(b'{') = stk.pop() else {
                        return false;
                    };
                }
                _ => return false,
            }
        }
        stk.is_empty()
    }
}