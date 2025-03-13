/// Solution to Is it a tree problem (https://www.spoj.com/problems/PT07Y/)

fn read_line_to_usize_vec() -> Vec<usize> {
   let mut string = String::new();
   let _ = std::io::stdin().read_line(&mut string);
   return string.trim().split(' ').map(|s| s.parse().unwrap()).collect();
}

fn read_graph(tree_size: usize) -> Vec<Vec<usize>> {

    let mut graph = vec![Vec::new(); tree_size + 1];
    for _ in 0..tree_size - 1 {
        let edge = read_line_to_usize_vec();
        graph[edge[0] - 1].push(edge[1] - 1);
        graph[edge[1] - 1].push(edge[0] - 1);
    }
    return graph;
}

fn traverse_graph(graph: &Vec<Vec<usize>>, node: usize, visited: &mut Vec<bool>, parent: usize) -> bool {

    visited[node] = true;

    for neighbor in &graph[node] {
        if visited[*neighbor] && *neighbor != parent {
            return false;
        }

        if *neighbor == parent {
            continue;
        }
        if !traverse_graph(graph, *neighbor, visited, node) {
            return false;
        }
    }
    return true;
}

fn main() {
    let test_cases_header = read_line_to_usize_vec();
    let nodes_cnt = test_cases_header[0];
    let edges_cnt = test_cases_header[1];
    if edges_cnt + 1 != nodes_cnt {
        println!("NO");
        return;
    }

    let graph = read_graph(nodes_cnt);
    let mut visited = vec![false; nodes_cnt];
    if !traverse_graph(&graph, 0, &mut visited, graph.len()) {
        println!("NO");
        return;
    }

    for node in visited {
        if !node {
            println!("NO");
            return;
        }
    }

    println!("YES");
}