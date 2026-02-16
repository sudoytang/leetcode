#[cfg(feature = "local")]
#[allow(unused)]
pub struct Solution;

#[cfg(feature = "local")]
#[allow(unused)]
use crate::ListNode;

#[cfg(feature = "local")]
#[allow(unused)]
use crate::TreeNode;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Copy, Clone, Eq, PartialEq)]
struct Vtx {
    cost: usize,
    id: i32,
}

impl Ord for Vtx {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost).then_with(|| self.id.cmp(&other.id))
    }
}

impl PartialOrd for Vtx {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn dijkstra(adj_list: &HashMap<i32, Vec<i32>>, start: i32) -> HashMap<i32, usize> {
    let mut dist: HashMap<i32, usize> = HashMap::new();
    let mut heap = BinaryHeap::new();
    dist.insert(start, 0);
    heap.push(Vtx { cost: 0, id: start });
    while let Some(Vtx { cost, id }) = heap.pop() {
        if let Some(&d) = dist.get(&id) {
            // there is already a valid distance, check if what we have is better
            if cost > d {
                // don't update because it is a better distance then we have now
                continue;
            }
        }
        if let Some(neighbors) = adj_list.get(&id) {
            for &nb in neighbors {
                let next_cost = cost + 1;
                let current_nb_dist = *dist.get(&nb).unwrap_or(&usize::MAX);
                if next_cost < current_nb_dist {
                    heap.push(Vtx { cost: next_cost, id: nb });
                    dist.insert(nb, next_cost);
                }
            }
        }
    }
    dist
}

pub fn is_pythagorean(d1: usize, d2: usize, d3: usize) -> bool {
    if d1 >= d2 && d1 >= d3 {
        d1 * d1 == d2 * d2 + d3 * d3
    } else if d2 >= d1 && d2 >= d3 {
        d2 * d2 == d1 * d1 + d3 * d3
    } else {
        d3 * d3 == d1 * d1 + d2 * d2
    }
}


impl Solution {
    pub fn special_nodes(n: i32, edges: Vec<Vec<i32>>, x: i32, y: i32, z: i32) -> i32 {
        let mut adj_list: HashMap<i32, Vec<i32>> = HashMap::new();

        for uv in edges.iter() {
            let u = uv[0];
            let v = uv[1];
            adj_list.entry(u).or_default().push(v);
            adj_list.entry(v).or_default().push(u);
        }

        let dist_x = dijkstra(&adj_list, x);
        let dist_y = dijkstra(&adj_list, y);
        let dist_z = dijkstra(&adj_list, z);

        let mut pythagorean_count = 0;
        for node in 0..n {
            if is_pythagorean(
                dist_x.get(&node).copied().unwrap(),
                dist_y.get(&node).copied().unwrap(),
                dist_z.get(&node).copied().unwrap()
            ) {
                pythagorean_count += 1;
            }
        }
        pythagorean_count
    }
}


#[cfg(test)]
mod test {
    #[test]
    fn test_basic() {
        let edges = vec![
            vec![0, 1],
            vec![0, 2],
            vec![0, 3],
        ];
        let res = super::Solution::special_nodes(4, edges, 1, 2, 3);
        assert_eq!(res, 3);
    }
}