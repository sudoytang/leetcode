use std::collections::{BTreeSet, HashMap};
use std::collections::hash_map::Entry::*;
use std::ops::Bound::*;
#[allow(unused)]
pub struct Solution;
#[allow(unused)]

impl Solution {
    pub fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
        let mut avail_days: BTreeSet<usize> = BTreeSet::new();
        let mut answer = vec![0; rains.len()];
        let mut full_lake: HashMap<i32, usize> = HashMap::new();
        for (i, rain_idx) in rains.into_iter().enumerate() {
            if rain_idx != 0 {
                answer[i] = -1;
                match full_lake.entry(rain_idx) {
                    Occupied(mut o) => {
                        let r = o.get_mut();
                        // we have to find a day to drain this lake
                        // after r and before today
                        match avail_days.range((Excluded(*r), Unbounded)).next() {
                            Some(&day) => {
                                *r = i;
                                answer[day] = rain_idx;
                                avail_days.remove(&day);
                            }
                            None => {
                                // No way to prevent a flood
                                return Vec::new();
                            }
                        }
                    }
                    Vacant(v) => {
                        // stay calm, the lake is now empty
                        v.insert(i);
                    }
                }
            } else {
                // no rain today, save this day for a raining day.
                answer[i] = 1;
                avail_days.insert(i);
            }
        }
        answer
    }
}