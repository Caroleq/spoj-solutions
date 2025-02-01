/// Solution to Horrible Queries (https://www.spoj.com/problems/HORRIBLE/).

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

fn get_parent(child: usize) -> usize {
    return child / 2;
}

fn is_right_child(node: usize) -> bool {
    return node == (node / 2) * 2 + 1;
}

fn is_left_child(node: usize) -> bool {
    return node == (node / 2) * 2;
}

fn get_siblings_idx(node: usize) -> usize {
    if is_right_child(node) {
        return node - 1;
    }
    else {
        return node + 1;
    }
}

fn sum_on_path(tree: &Vec<(u64,u64)>, idx: usize, segment_length: usize) -> u64 {
    let mut sum = 0;
    let mut idx = idx;
    while idx != 0 {
        sum += tree[idx].0;
        idx /= 2;
    }
    return sum * (segment_length as u64);
}

fn find_max_subsum(left: usize, right: usize, tree: &Vec<(u64,u64)>) -> u64 {
    let offset = tree.len() / 2;
    let mut left_idx = offset + left - 1;
    let mut right_idx = offset + right - 1;

    if left_idx == right_idx {
        return tree[left_idx].0 + sum_on_path(tree, get_parent(left_idx), 1);
    }

    let mut sum_on_range = 0; 

    let mut level = 1;
    let mut left_child_cnt = 1;
    let mut right_child_cnt = 1;
    while left_idx != right_idx {

        sum_on_range += tree[left_idx].0 * left_child_cnt;
        sum_on_range += tree[right_idx].0 * right_child_cnt;

        if get_siblings_idx(left_idx) != right_idx {
            
            if is_left_child(left_idx) {
                sum_on_range += tree[get_siblings_idx(left_idx)].1;
                left_child_cnt += level;
            }
            
            if is_right_child(right_idx) {
                sum_on_range += tree[get_siblings_idx(right_idx)].1;
                right_child_cnt += level;
            }
        }

        left_idx = get_parent(left_idx);
        right_idx = get_parent(right_idx);
        level *= 2;
    }

    return sum_on_range + sum_on_path(tree, left_idx, right - left + 1);
}

fn get_min_power_geq(number: usize) -> usize {
    let mut power_of_two = 1;
    while power_of_two < number {
        power_of_two = power_of_two * 2;
    }
    return power_of_two;
}

fn update_left_siblings_node_if_present(segment_tree: &mut Vec<(u64,u64)>, idx: usize, value: u64, child_cnt: usize) {
    let left_child_idx = 2 * get_parent(idx);
    if left_child_idx == idx {
        return;
    }
    segment_tree[left_child_idx].0 += value;
    segment_tree[left_child_idx].1 += value * child_cnt as u64; 
}

fn update_right_siblings_node_if_present(segment_tree: &mut Vec<(u64,u64)>, idx: usize, value: u64, child_cnt: usize) {
    let right_child_idx = 2 * get_parent(idx) + 1;
    if right_child_idx == idx {
        return;
    }
    segment_tree[right_child_idx].0 += value;
    segment_tree[right_child_idx].1 += value * child_cnt as u64;
}

fn insert_into_segment_tree(segment_tree: &mut Vec<(u64,u64)>, left_idx: usize, right_idx: usize, value: u64) {
    let offset = segment_tree.len() / 2;
    let mut left_idx = left_idx + offset - 1;
    let mut right_idx = right_idx + offset - 1;

    if left_idx == right_idx {
        segment_tree[left_idx].0 += value;
        segment_tree[left_idx].1 += value;

        let mut child_cnt = 1;
        while left_idx != 1 {
            left_idx = get_parent(left_idx);
            child_cnt *= 2;
            segment_tree[left_idx].1 = segment_tree[2 * left_idx].1 + segment_tree[2 * left_idx + 1].1 + segment_tree[left_idx].0 * child_cnt as u64;
        }
        return;
    }

    segment_tree[left_idx].0 += value;
    segment_tree[right_idx].0 += value;

    segment_tree[left_idx].1 += value;
    segment_tree[right_idx].1 += value;

    let mut child_cnt = 1;

    while get_parent(left_idx) != get_parent(right_idx) {

        update_left_siblings_node_if_present(segment_tree, right_idx, value, child_cnt);
        update_right_siblings_node_if_present(segment_tree, left_idx, value, child_cnt);

        left_idx = get_parent(left_idx);
        right_idx = get_parent(right_idx);
        child_cnt *= 2;

        segment_tree[left_idx].1 = segment_tree[2 * left_idx].1 + segment_tree[2 * left_idx + 1].1 + segment_tree[left_idx].0 * child_cnt as u64;
        segment_tree[right_idx].1 = segment_tree[2 * right_idx].1 + segment_tree[2 * right_idx + 1].1 + segment_tree[right_idx].0 * child_cnt as u64;
    }

    while left_idx != 1 {
        left_idx = get_parent(left_idx);
        child_cnt *= 2;
        segment_tree[left_idx].1 = segment_tree[2 * left_idx].1 + segment_tree[2 * left_idx + 1].1 + segment_tree[left_idx].0 * child_cnt as u64;
    }
}

fn create_segment_tree(max_data: usize) -> Vec<(u64,u64)> {
    let segment_tree_len = 2 * get_min_power_geq(max_data) + 1;
    let segment_tree = vec![(0,0) ; segment_tree_len];  
    return segment_tree;
}

fn main() {

    let cases_cnt = read_line_to_usize();

    for _ in 0..cases_cnt {
        let test_case_header = read_line_to_usize_vec();

        let tree_length = test_case_header[0];
        let queries_cnt = test_case_header[1];
        let mut tree = create_segment_tree(tree_length);
        for _ in 0..queries_cnt {
            let query = read_line_to_usize_vec();
            let query_type = query[0];
            if query_type == 0 {
                insert_into_segment_tree(&mut tree, query[1], query[2], query[3] as u64);
            }
            else {
                let sum_on_range = find_max_subsum(query[1], query[2], &tree);
                println!("{sum_on_range}", sum_on_range=sum_on_range);
            }
        }
    }
}