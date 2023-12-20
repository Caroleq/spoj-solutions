/// Solution to DEV_CP (Ambitious XOXO) problem (https://www.spoj.com/problems/DEV_CP/)

fn read_line_to_usize_vec() -> Vec<usize> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().split(' ').map(|s| s.parse().unwrap()).collect();
}

fn read_line_to_u64_vec() -> Vec<u64> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().split(' ').map(|s| s.parse().unwrap()).collect();
}

fn get_min_power_geq(number: usize) -> usize {
    let mut power_of_two = 1;
    while power_of_two < number {
        power_of_two <<= 1;
    }
    return power_of_two;
}

fn create_segment_tree(efficiencies: &Vec<u64>) -> Vec<u64> {

    let segment_tree_len = 2 * get_min_power_geq(efficiencies.len()) + 1;
    let mut segment_tree = vec![0; segment_tree_len];

    let mut idx = segment_tree_len/2;
    for efficiency in efficiencies {
        segment_tree[idx] = *efficiency;
        idx += 1;
    }

    for idx in (1..(segment_tree_len/2)).rev() {
        segment_tree[idx] = segment_tree[2 * idx] + segment_tree[2 * idx + 1];
    }

    return segment_tree;
}

fn insert_into_segment_tree(segment_tree: &mut Vec<u64>, position: usize, number: u64) {
    let mut position = position + segment_tree.len()/2 - 1;
    let old_value = segment_tree[position];
    while position > 0 {
        segment_tree[position] += number;
        segment_tree[position] -= old_value;
        position >>= 1;
    }
}

fn find_range_sum(segment_tree: &Vec<u64>, left: usize, right: usize) -> u64 {
    
    let mut left = segment_tree.len()/2 + left - 1;
    let mut right = segment_tree.len()/2 + right - 1;
    if left == right {
        return segment_tree[right];
    }

    let mut sum = segment_tree[left] + segment_tree[right];
    let mut prev_left = left;
    let mut prev_right = right;
    left >>= 1;
    right >>= 1;
    while left != right {
        if prev_left != 2 * left + 1 {
            sum += segment_tree[2 * left + 1];
        }

        if prev_right != 2 * right {
            sum += segment_tree[2 * right];
        }

        prev_left = left;
        prev_right = right;
        left >>= 1;
        right >>= 1;
    }
    return sum;
}

fn main() {

    let first_line = read_line_to_usize_vec();
    let queries_cnt = first_line[1];

    let efficiencies = read_line_to_u64_vec();

    let mut segment_tree = create_segment_tree(&efficiencies);

    for _ in 0..queries_cnt {
        let query = read_line_to_usize_vec();
        let command = query[0];

        if command == 1 {
            insert_into_segment_tree(&mut segment_tree, query[1], query[2] as u64);
        }
        else {
            let range_sum = find_range_sum(&segment_tree, query[1], query[2]);
            println!("{range_sum}", range_sum=range_sum);
        }
    }
}