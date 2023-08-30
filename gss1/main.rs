/// Solution to GSS1 Problem (https://www.spoj.com/problems/GSS1/)
/// Solution was inspired by https://cp-algorithms.com/data_structures/segment_tree.html#finding-subsegments-with-the-maximal-sum

fn read_line_to_i32() -> i32 {
   let mut string = String::new();
   let _ = std::io::stdin().read_line(&mut string);
   return string.trim().parse().expect("Input not an integer");
}

fn read_line_to_usize_vec() -> Vec<usize> {
   let mut string = String::new();
   let _ = std::io::stdin().read_line(&mut string);
   return string.trim().split(' ').map(|s| s.parse().unwrap()).collect();
}

fn read_line_to_i32_vec() -> Vec<i32> {
   let mut string = String::new();
   let _ = std::io::stdin().read_line(&mut string);
   return string.trim().split(' ').map(|s| s.parse().unwrap()).collect();
}

fn two_power_greater_than(n: usize) -> usize {
    let mut pow = 1;
    while pow < n {
        pow = pow << 1; 
    }
    return pow;
}

fn combine(left: (i32, i32, i32, i32), right: (i32, i32, i32, i32)) -> (i32, i32, i32, i32) {

    let sum = left.0 + right.0;
    let prefix = std::cmp::max(left.1, left.0 + right.1);
    let suffix = std::cmp::max(right.2, right.0 + left.2);
    let subsum = std::cmp::max(left.3, std::cmp::max(right.3, left.2 + right.1));
   
    return (sum, prefix, suffix, subsum);
}

fn build_tree(sequence: &Vec<i32>) -> Vec<(i32, i32, i32, i32)> {

    let pow_size = two_power_greater_than(sequence.len());
    let tree_size = 2 * pow_size;

    let mut tree: Vec<(i32, i32, i32, i32)> = vec![(0, 0, 0, 0); tree_size];
    let tree_offset = pow_size;

    for idx in 0..sequence.len() {
        tree[idx + tree_offset] = (sequence[idx], sequence[idx], sequence[idx], sequence[idx]);
    }

    let mut idx = tree_offset - 1;
    while idx != 0 {
        tree[idx] = combine(tree[2 * idx], tree[2 * idx + 1]);
        idx -= 1;
    }

    return tree;
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

fn find_max_subsum(left: usize, right: usize, tree: &Vec<(i32, i32, i32, i32)>) -> i32 {
    let offset = tree.len() / 2;
    let mut left_idx = offset + left - 1;
    let mut right_idx = offset + right - 1;

    if left_idx == right_idx {
        return tree[left_idx].3;
    }

    let mut left_max = tree[left_idx];
    let mut right_max = tree[right_idx];

    while left_idx != right_idx {
        if is_left_child(left_idx) && get_siblings_idx(left_idx) != right_idx {
            left_max = combine(left_max, tree[get_siblings_idx(left_idx)]);
        }
        
        if is_right_child(right_idx) && get_siblings_idx(right_idx) != left_idx {
            right_max = combine(tree[get_siblings_idx(right_idx)], right_max);
        }

        left_idx = get_parent(left_idx);
        right_idx = get_parent(right_idx);
    }

    return combine(left_max, right_max).3;
}

fn main() {
    read_line_to_i32();

    let sequence = read_line_to_i32_vec();
    let tree = build_tree(&sequence);

    let m = read_line_to_i32();
    for _ in 0..m {
        let query = read_line_to_usize_vec();

        let left = query[0];
        let right = query[1];

        let max_subsum = find_max_subsum(left, right, &tree);
        println!("{max_subsum}", max_subsum=max_subsum);

    }
}
