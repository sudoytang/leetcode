#[cfg(feature = "local")]
#[allow(unused)]
pub struct Solution;

#[cfg(feature = "local")]
#[allow(unused)]
use crate::ListNode;

#[cfg(feature = "local")]
#[allow(unused)]
use crate::TreeNode;


#[allow(unused)]
impl Solution {
    pub fn recur(n: usize, used_left: usize, cur: &mut String, res: &mut Vec<String>) {
        let used_right = cur.len() - used_left;
        if used_left == n {
            let orig_len = cur.len();
            for _ in used_right..n {
                cur.push(')');
            }
            res.push(cur.clone());
            cur.truncate(orig_len);
            return;
        }
        // add a new paren
        cur.push('(');
        Self::recur(n, used_left + 1, cur, res);
        cur.pop();
        if used_right < used_left {
            cur.push(')');
            Self::recur(n, used_left, cur, res);
            cur.pop();
        }
    }
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut res = Vec::new();
        Self::recur(n as usize, 0, &mut String::new(), &mut res);
        res
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_q22() {
        let mut res = super::Solution::generate_parenthesis(3);
        res.sort_unstable();
        let mut gold: Vec<String> = ["((()))","(()())","(())()","()(())","()()()"].into_iter().map(|s| s.into()).collect();
        gold.sort_unstable();
        assert_eq!(gold, res);
    }
}