#![allow(dead_code)]

// O(h * w) returns a graph of a maze.
fn create_maze_graph(h: usize, w: usize, s: &Vec<Vec<char>>, wall: char) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; h * w];

    for i in 0..h {
        let v = &s[i];
        for j in 0..w {
            let ch = v[j];
            if ch == wall {
                continue;
            }

            let p = w * i + j;
            // up
            if i > 0 && s[i - 1][j] != wall {
                graph[p].push(w * (i - 1) + j);
            }
            // down
            if i != (h - 1) && s[i + 1][j] != wall {
                graph[p].push(w * (i + 1) + j);
            }
            // left
            if j > 0 && s[i][j - 1] != wall {
                graph[p].push(p - 1);
            }
            // right
            if j != (w - 1) && s[i][j + 1] != wall {
                graph[p].push(p + 1);
            }
        }
    }

    graph
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn test_create_maze_graph() {
        let h = 3;
        let w = 3;
        let s = vec![
            vec!['.', '.', '.'],
            vec!['.', '#', '.'],
            vec!['.', '.', '.'],
        ];
        let wall = '#';
        let mut graph = create_maze_graph(h, w, &s, wall);

        for i in 0..graph.len() {
            graph[i].sort();
        }

        assert_eq!(
            graph,
            vec![
                vec![1, 3],
                vec![0, 2],
                vec![1, 5],
                vec![0, 6],
                vec![],
                vec![2, 8],
                vec![3, 7],
                vec![6, 8],
                vec![5, 7],
            ]
        );
    }
}

fn main() {}
