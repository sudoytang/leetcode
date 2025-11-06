use std::{collections::{BinaryHeap, HashSet}, usize};


#[cfg(feature = "local")]
#[allow(unused)]
pub struct Solution;

#[derive(Clone)]
struct Node {
    net_id: usize,
    edges: HashSet<usize>,
    stat: bool,
}

impl Node {
    pub fn new() -> Self {
        Self {
            net_id: usize::MAX,
            edges: Default::default(),
            stat: true,
        }
    }
}


impl Solution {
    fn dfs(graph: &mut Vec<Node>) -> usize {
        let mut next_net_id = 0;
        let mut stack = Vec::new();
        for i in (0..graph.len()).rev() {
            if graph[i].net_id != usize::MAX {
                // already visited
                continue;
            }
            stack.push(i);
            while let Some(i) = stack.pop() {
                graph[i].net_id = next_net_id;
                for j in graph[i].edges.iter().copied() {
                    if graph[j].net_id == usize::MAX {
                        stack.push(j);
                    } else {
                        assert_eq!(next_net_id, graph[j].net_id);
                    }
                }
            }
            next_net_id += 1;
        }
        next_net_id
    }
    #[allow(unused)]
    pub fn process_queries(c: i32, connections: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut graph: Vec<Node> = Vec::with_capacity(c as usize);
        for _ in 0..c {
            graph.push(Node::new());
        }
        for conn in connections {
            let n1 = conn[0] - 1;
            let n2 = conn[1] - 1;
            graph[n1 as usize].edges.insert(n2 as usize);
            graph[n2 as usize].edges.insert(n1 as usize);
        }
        let cc_count = Self::dfs(&mut graph);
        // let graph = graph;
        let mut cc_heap: Vec<BinaryHeap<usize>> = graph.iter().enumerate().fold(vec![BinaryHeap::new(); cc_count], |mut cc, (i, n)| {
            cc[n.net_id].push(200000 - i);
            cc
        });

        let mut res = Vec::new();
        for q in queries {
            let kind = q[0];
            let id = q[1] as usize - 1;
            match kind {
                1 => {
                    let node = &graph[id];
                    let heap = &mut cc_heap[node.net_id];
                    if node.stat {
                        res.push(id as i32 + 1);
                    } else {
                        while heap.peek().is_some_and(|&n| graph[200000 - n].stat == false) {
                            heap.pop();
                        }
                        if let Some(&n) = heap.peek() {
                            res.push(200000 - n as i32 + 1);
                        } else {
                            res.push(-1);
                        }
                    }
                }
                2 => {
                    graph[id].stat = false;
                }
                _ => unreachable!()
            }
        }
        res
    }
}
