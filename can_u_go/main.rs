/// Solution to Can Adnan go? Problem (https://www.spoj.com/problems/CAN_U_GO/)

use std::collections::VecDeque;


fn read_line_to_usize() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}

fn read_line_to_usize_vec() -> Vec<usize> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().split(' ').map(|s| s.parse().unwrap()).collect();
}

fn read_lines_to_graph(strings_cnt: usize) -> Vec<Vec<usize>> {
    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); strings_cnt + 2];
    for _ in 0..strings_cnt {
        let mut string = String::new();
        let _ = std::io::stdin().read_line(&mut string);
        let edge = string.trim().split(' ').map(|s| s.parse().unwrap()).collect::<Vec<usize>>();
        graph[edge[0] as usize].push(edge[1]);
        graph[edge[1] as usize].push(edge[0]);
    }
    return graph;
}

static mut COMPONENT_CNT: u64 = 1;

fn compute_connected_components(node: usize, graph: &Vec<Vec<usize>>, behavioral_values: &Vec<usize>) ->  Vec<u64> {

    let mut components = vec![0; graph.len()];

    unsafe{
        components[node] = COMPONENT_CNT;
    }

    let mut deque: VecDeque<(usize, usize)> = VecDeque::new();
    for neighbour in graph[node].iter() {
        deque.push_back((node, *neighbour));
    }

    while !deque.is_empty() {
        let (u, v) = match deque.pop_front() {
            Some((u, v)) => (u, v),
            None => (0, 0)
        };

        if behavioral_values[u] != behavioral_values[v] {
            unsafe { 
                COMPONENT_CNT += 1; 
                components[v] = COMPONENT_CNT;
            }
        }
        else {
            components[v] = components[u];
        }

        for neighbour in graph[v].iter() {
            if *neighbour != u {
                deque.push_back((v, *neighbour));
            }
        }
    }
    return components;
}

fn update_connected_component(node: usize, graph: &Vec<Vec<usize>>, components: &mut Vec<u64>, new_id: u64) {

    let old_id = components[node];
    components[node] = new_id;

    let mut deque: VecDeque<(usize, usize)> = VecDeque::new();
    for neighbour in graph[node].iter() {

        if components[*neighbour as usize] == old_id { 
            deque.push_back((node, *neighbour));
        }
    }

    while !deque.is_empty() {
        let (u, v) = match deque.pop_front() {
            Some((u, v)) => (u, v),
            None => (0, 0)
        };

        components[v] = new_id;

        for neighbour in graph[v].iter() {
            if *neighbour != u && components[*neighbour as usize] == old_id {
                deque.push_back((v, *neighbour));
            }
        }
    }
}

fn update_bahavioral_value(node: usize, graph: &Vec<Vec<usize>>, behavioral_values: &Vec<usize>, old_behavioural_value: usize, components: &mut Vec<u64>) {

    let mut to_merge = Vec::new();
    let mut to_separate = Vec::new();

    unsafe {
        COMPONENT_CNT += 1; 
        components[node] = COMPONENT_CNT;
    }

    for neighbour in graph[node].iter() {

        if behavioral_values[*neighbour as usize] == behavioral_values[node] {
            to_merge.push(*neighbour);
        }
        else if behavioral_values[*neighbour as usize] == old_behavioural_value {
            to_separate.push(*neighbour);
        }
    }

    if to_separate.len() > 1 {

        for node in to_separate.iter() {
            unsafe { 
                COMPONENT_CNT += 1; 
                update_connected_component(*node, graph, components, COMPONENT_CNT);
                components[*node] = COMPONENT_CNT;
            }
        }
    }

    if to_merge.len() > 0 {
        let common_id = components[to_merge[0]];

        for idx in 1..to_merge.len() {
            update_connected_component(to_merge[idx], graph, components, common_id);
        }

        components[node] = common_id;
    }
}

fn main() {
    let planets_cnt = read_line_to_usize();
    let mut behavioral_values = read_line_to_usize_vec();
    behavioral_values.insert(0, 0);
    let graph = read_lines_to_graph(planets_cnt - 1);
    let queries_cnt = read_line_to_usize();
    
    let mut components = compute_connected_components(1, &graph, &behavioral_values);

    for _ in 0..queries_cnt {
        let query = read_line_to_usize_vec();
        let command = query[0];
        if command == 1 {
            if components[query[1]] ==  components[query[2]] {
                println!("Yes");
            }
            else {
                println!("No");
            }
        }
        else {
            let old_value = behavioral_values[query[1]];
            if old_value != query[2] {
                behavioral_values[query[1]] = query[2];
                update_bahavioral_value(query[1], &graph, &behavioral_values, old_value, &mut components);
            }
        }
    }
}