// Solution to Counting Child Problem (https://www.spoj.com/problems/CTTC/)

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


fn parse_traversal_to_tree(traversal: &Vec<usize>) -> Vec<Vec<usize>> {
    let mut tree = vec![Vec::new(); traversal.len()/2 + 1];
    let mut parents = vec![0; traversal.len() / 2 + 1];

    let mut visited = vec![false; traversal.len() / 2 + 1];

    let mut current_node = 0;

    for node in traversal.iter() {

        if visited[*node] {
            current_node = parents[current_node];
        }
        else {
            visited[*node] = true;
            tree[current_node].push(*node);
            parents[*node] = current_node;
            current_node = *node;
        }
    }


    return tree;
}


fn main() {
    let test_cases_cnt = read_line_to_usize();

    for case_idx in 0..test_cases_cnt {
        let tree_size = read_line_to_usize();

        let tree_traversal = read_line_to_usize_vec();

        let tree = parse_traversal_to_tree(&tree_traversal);

        println!("Case {case_no}:", case_no=case_idx+1);
        for node_idx in 1..tree_size + 1 {
            println!("{node_idx} -> {children_cnt}", node_idx=node_idx, children_cnt=tree[node_idx].len());
        }
        println!("");
    }
}