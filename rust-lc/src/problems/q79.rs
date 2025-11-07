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
    pub fn recur(board: &mut Vec<Vec<char>>, word: &str, i: usize, pos: (usize, usize)) -> bool {
        let rows = board.len();
        let cols = board[0].len();
        let (y, x) = pos;
        if board[y][x] != word.as_bytes()[i] as char {
            return false;
        }
        // match!
        board[y][x] = (128 | board[y][x] as u8) as char;
        if i + 1 == word.len() {
            return true;
        }
        if y + 1 < rows && Self::recur(board, word, i + 1, (y + 1, x)) {
            return true;
        }
        if y > 0 && Self::recur(board, word, i + 1, (y - 1, x)) {
            return true;
        }
        if x + 1 < cols && Self::recur(board, word, i + 1, (y, x + 1)) {
            return true;
        }
        if x > 0 && Self::recur(board, word, i + 1, (y, x - 1)) {
            return true;
        }

        // restore
        board[y][x] = (board[y][x] as u8 & !128u8) as char;
        return false;
    }
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let mut board = board;
        let rows = board.len();
        let cols = board[0].len();
        for y in 0..rows {
            for x in 0..cols {
                if Self::recur(&mut board, &word, 0, (y, x)) {
                    return true;
                }
            }
        }
        return false;
    }
}