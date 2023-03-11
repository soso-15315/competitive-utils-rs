#![allow(dead_code)]

// O(n + m) returns a shortest distance from v to each node.
fn bfs(v: usize, graph: &Vec<Vec<usize>>) -> Vec<isize> {
    let mut dist = vec![-1; graph.len()];
    let mut queue = std::collections::VecDeque::<usize>::new();
    queue.push_front(v);
    dist[v] = 0;

    while !queue.is_empty() {
        let pos = *queue.front().unwrap();
        queue.pop_front().unwrap();

        for i in 0..graph[pos].len() {
            let nex = graph[pos][i];
            if dist[nex] == -1 {
                dist[nex] = dist[pos] + 1;
                queue.push_back(nex);
            }
        }
    }

    dist
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs() {
        let graph = vec![
            vec![1, 2],
            vec![0, 2],
            vec![0, 1, 3],
            vec![2, 4, 5],
            vec![3, 5],
            vec![3, 4],
            vec![],
        ];
        let dist = bfs(0, &graph);
        assert_eq!(dist, vec![0, 1, 1, 2, 3, 3, -1]);
    }
}

fn main() {}
