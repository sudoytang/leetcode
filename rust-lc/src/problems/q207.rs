use super::Solution;


#[derive(PartialEq, Eq, Clone, Copy)]
enum VertexState {
    Unvisited,
    Visiting,
    Visited,
}

use VertexState::*;

impl Solution {
    fn can_finish_dfs(visited: &mut [VertexState], prerequisites: &Vec<Vec<usize>>, i: usize) -> bool {
        visited[i] = Visiting;
        for &preq in &prerequisites[i] {
            if visited[preq] == Visited {
                continue;
            }
            if visited[preq] == Visiting {
                return false;
            }
            if !Self::can_finish_dfs(visited, prerequisites, preq) {
                return false;
            }
        }
        visited[i] = Visited;
        return true;
    }
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let num_courses = num_courses as usize;
        let prerequisites: Vec<Vec<usize>> = prerequisites.into_iter().map(|v| {
            (v[0] as usize, v[1] as usize)
        }).fold(vec![vec![]; num_courses], |mut acc, (i, preq)| {
            acc[i].push(preq);
            acc
        });

        let mut visited = vec![Unvisited; num_courses];
        for i in 0..num_courses {
            if visited[i] == Visited {
                continue;
            }
            if !Self::can_finish_dfs(visited.as_mut_slice(), &prerequisites, i) {
                return false;
            }
        }

        true
    }
}