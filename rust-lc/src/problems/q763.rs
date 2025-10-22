use super::Solution;

enum MergeRange {
    DisJoint((usize, usize)),
    Merged((usize, usize))
}


impl MergeRange {
    fn merge(mut rng1: (usize, usize), mut rng2: (usize, usize)) -> MergeRange {
        if rng1.0 > rng2.0 {
            std::mem::swap(&mut rng1, &mut rng2);
        }
        // now rng1.0 <= rng2.0
        if rng1.1 < rng2.0 {
            // [1, 4] + [6, 7]
            MergeRange::DisJoint(rng2)
        } else {
            MergeRange::Merged((rng1.0, usize::max(rng2.1, rng1.1)))
        }
    }
}



impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut word_range = [(usize::MAX, usize::MAX); 26];

        for (i, ch) in s.bytes().enumerate() {
            let (start, end) = &mut word_range[(ch - b'a') as usize];
            if *start == usize::MAX {
                *start = i;
            }
            *end = i;
        }
        word_range.sort_unstable_by_key(|(l, _)| *l);
        if word_range[0] == (usize::MAX, usize::MAX) {
            return vec![s.len() as i32];
        }
        let mut ranges = Vec::new();
        ranges.push(word_range[0]);
        for &rng in word_range.iter().skip(1) {
            if rng == (usize::MAX, usize::MAX) {
                continue;
            }
            let last_mut = ranges.last_mut().unwrap();
            match MergeRange::merge(*last_mut, rng) {
                MergeRange::DisJoint(b) => ranges.push(b),
                MergeRange::Merged(m) => *last_mut = m
            }
        }

        ranges.into_iter().map(|(l, r)| (r + 1 - l) as i32).collect()
    }
}


#[cfg(test)]
mod test {
    use crate::problems::Solution;

    #[test]
    fn test_partition_labels_eg0() {
        let s = String::from("ababcbacadefegdehijhklij");
        let res = Solution::partition_labels(s);
        assert_eq!(res, [9, 7, 8]);
    }
}