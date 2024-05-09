/// Solution to Transitive Closure Problem (https://www.spoj.com/problems/TRANCLS/)

use std::collections::HashSet;
use std::collections::HashMap;

fn read_line_to_usize() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}

fn read_line_to_u32_vec() -> Vec<u32> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().split(' ').map(|s| s.parse().unwrap()).collect();
}

fn count_edges_to_make_transitive(edges: &mut HashMap<u32, HashSet<u32>>) -> usize {

    let mut edges_to_add_cnt = 0;

    loop {
        let mut changed = false;
        let map_to_check = edges.clone();

        for (v1, v1_neighbors) in map_to_check.iter() {
            for v2 in v1_neighbors {
                if !map_to_check.contains_key(&v2) {
                    continue;
                }

                let v2_neighbors = map_to_check.get(&v2).unwrap();
                for v3 in v2_neighbors {

                    if !v1_neighbors.contains(&v3) && !edges.get(&v1).unwrap().contains(v3) {
                        edges.get_mut(&v1).unwrap().insert(*v3);
                        changed = true;
                        edges_to_add_cnt += 1;
                    }
                }
            }
        }

        if !changed {
            break;
        }
    }

    return edges_to_add_cnt;
}

fn main() {
    let test_cases_cnt = read_line_to_usize();

    for idx in 0..test_cases_cnt {

        let edges_cnt = read_line_to_usize();

        let mut edges = HashMap::new(); 
        for _ in 0..edges_cnt {
            let edge = read_line_to_u32_vec();
            if !edges.contains_key(&edge[0]) {
                edges.insert(edge[0], HashSet::new());
            }
            edges.get_mut(&edge[0]).unwrap().insert(edge[1]);
        }

        let edges_to_make_transitive = count_edges_to_make_transitive(&mut edges);
        println!("Case #{case_id}: {result}", case_id=idx + 1, result=edges_to_make_transitive);
    }
}