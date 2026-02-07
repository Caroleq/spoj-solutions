/// Solution to A Bug’s Life Problem (https://www.spoj.com/problems/BUGLIFE/)


fn read_line_to_u64() -> u64 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    return input.trim().parse::<u64>().unwrap();
}

fn read_line_to_u64_vec() -> Vec<u64> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    return input.trim().split(' ').map(|s| s.parse().unwrap()).collect();
}

fn bfs(graph: &Vec<Vec<u64>>) -> bool {

    let mut colors = vec![0; graph.len()];
    
    for node_idx in 0..graph.len() {
        if colors[node_idx] == 0 {
            if !bfs_from_node(graph, node_idx, &mut colors) {
                return false;
            }
        }
    }
    return true;
}

fn bfs_from_node(graph: &Vec<Vec<u64>>, start_node: usize, colors: &mut Vec<u8>) -> bool {    
    let mut queue = std::collections::VecDeque::new();
    colors[start_node] = 1;
    queue.push_back(start_node);

    while let Some(node) = queue.pop_front() {
        let current_color = colors[node];
        let next_color = if current_color == 1 { 2 } else { 1 };

         for neighbor in &graph[node] {
            if colors[*neighbor as usize] == 0 {
                colors[*neighbor as usize] = next_color;
                queue.push_back(*neighbor as usize);
            }
            else if colors[*neighbor as usize] == current_color {
                // Found a conflict, the graph is not bipartite
                return false;
            }
        }
    }

    return true;
}


fn main() {
    let test_cases_cnt = read_line_to_u64();
    for test_idx in 0..test_cases_cnt {
        let first_line = read_line_to_u64_vec();
        let bugs_cnt = first_line[0];
        let interactions_cnt = first_line[1];

        let mut graph = vec![vec![]; bugs_cnt as usize];
        for _ in 0..interactions_cnt {
            let interaction = read_line_to_u64_vec();
            let a = interaction[0] - 1;
            let b = interaction[1] - 1;
            graph[a as usize].push(b);
            graph[b as usize].push(a);
        }

        println!("Scenario #{test_no}:", test_no=test_idx + 1);
        if bfs(&graph) {
            println!("No suspicious bugs found!");
        }
        else {
            println!("Suspicious bugs found!");
        }
    }
}