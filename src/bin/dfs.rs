#![allow(dead_code)]

// O(n + m) returns a list of nodes in the order of the depth-first search.
fn dfs(start: usize, graph: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut visited: Vec<bool> = vec![false; graph.len()];
    let mut route: Vec<usize> = vec![];
    _dfs(start, graph, &mut visited, &mut route);
    route
}

fn _dfs(v: usize, graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>, route: &mut Vec<usize>) {
    visited[v] = true;
    route.push(v);

    for &w in graph[v].iter() {
        if !visited[w] {
            _dfs(w, graph, visited, route);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs() {
        let graph = vec![vec![1, 2], vec![0, 3], vec![0, 3], vec![1, 2], vec![]];
        let start = 0;
        let route = dfs(start, &graph);
        assert_eq!(route, vec![0, 1, 3, 2]);
    }
}

fn main() {}
