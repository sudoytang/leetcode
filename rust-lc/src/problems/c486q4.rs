#[cfg(feature = "local")]
#[allow(unused)]
pub struct Solution;

#[cfg(feature = "local")]
#[allow(unused)]
use crate::ListNode;

#[cfg(feature = "local")]
#[allow(unused)]
use crate::TreeNode;


fn comb(n: u32, k: u32) -> u64 {
    // calc C(n, k)
    if k > n {
        return 0;
    }

    let k = if k > n / 2 { n - k } else { k };

    let mut res = 1;
    for i in 0..k {
        res = (res as u64 * (n - i) as u64 / (i + 1) as u64) as u64
    }

    res

}

impl Solution {
    pub fn nth_smallest(n: i64, k: i32) -> i64 {

        let mut rank = n as u64 - 1;

        let mut result: u64 = 0;
        let mut k = k as u32;
        for bit_index in (0..55).rev() {
            if k == 0 {
                break;
            }

            if (bit_index + 1) == k {
                result |= (1u64 << (bit_index + 1)) - 1;
                break;
            }
            let count_if_place_zero = comb(bit_index, k);
            if rank < count_if_place_zero {

            } else {
                result |= 1u64 << bit_index;
                rank -= count_if_place_zero;
                k -= 1;
            }
        }
        result as i64
    }
}