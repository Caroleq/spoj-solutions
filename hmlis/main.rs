/// Solution to HowManyLis (https://www.spoj.com/problems/HMLIS/)
/// 
use std::collections::HashMap;

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

fn insert_into_segment_tree(tree: &mut Vec<(usize,usize)>, position: usize, number: (usize, usize)) {
    let mut idx = position + tree.len() / 2 - 1;
    if tree[idx].0 != number.0 {
        tree[idx] = number;
    }
    else {
        tree[idx].1 += number.1;
    }
    
    while idx > 1 {
        idx >>= 1;
        let max_elem = std::cmp::max(tree[2 * idx].0, tree[2 * idx + 1].0);
        let mut expected_cnt = 0;
        if tree[2 * idx].0 == max_elem {
            expected_cnt += tree[2 * idx].1;
        }
        
        if tree[2 * idx + 1].0 == max_elem {
            expected_cnt += tree[2 * idx + 1].1;
        }
        expected_cnt %= 1000000007;
        
        if tree[idx] == (max_elem, expected_cnt) {
            return;
        }
        tree[idx] = (max_elem, expected_cnt);
    }
}

fn is_left_child(parent: usize, child: usize) -> bool {
    return 2 * parent == child;
}

fn count_max_up_to(segment_tree: &Vec<(usize,usize)>, max_number: usize) -> (usize, usize) {
    if max_number == 0 {
        return (0, 0);
    }
    let mut idx = max_number + segment_tree.len() / 2 - 1;

    let mut maximum = segment_tree[idx];
    while idx != 1 {
        let last_idx = idx;
        idx >>= 1;
        if is_left_child(idx, last_idx) {
           continue; 
        }

        if segment_tree[2 * idx].0 > maximum.0 {
            maximum = segment_tree[2 * idx];
        }
        else if segment_tree[2 * idx].0 == maximum.0 {
            maximum.1 += segment_tree[2 * idx].1;
        }
    }
    return maximum;
}

fn get_min_power_geq(number: usize) -> usize {
    let mut power_of_two = 1;
    while power_of_two < number {
        power_of_two <<= 1;
    }
    return power_of_two;
}

fn create_segment_tree(sequence_len: usize) -> Vec<(usize,usize)> {
    let tree_len = (get_min_power_geq(sequence_len) << 1) + 1;
    return vec![(0, 0); tree_len]; 
}

fn compute_lis_len_and_cnt(sequence: &Vec<usize>) -> (usize, usize) {

    let mut sorted_unique_seq = sequence.to_vec();
    sorted_unique_seq.sort_unstable();
    sorted_unique_seq.dedup();
    let mut mapping = HashMap::with_capacity(sorted_unique_seq.len());
    for idx in 0..sorted_unique_seq.len() {
        mapping.insert(sorted_unique_seq[idx], idx + 1);
    }

    if sequence.len() <= 1 {
        return (sequence.len(), sequence.len());
    }

    let mut tree = create_segment_tree(sorted_unique_seq.len());
    insert_into_segment_tree(&mut tree, mapping[&sequence[0]], (1, 1));
    for element in &sequence[1..] {
        let mut lis = count_max_up_to(&tree, mapping[element] - 1);
        if lis == (0, 0) {
            lis = (0, 1);
        }
        insert_into_segment_tree(&mut tree, mapping[element], (lis.0 + 1, lis.1));
    }
    return tree[1];
}

fn main() {
    let _sequence_size = read_line_to_usize();
    let sequence = read_line_to_usize_vec();

    let result = compute_lis_len_and_cnt(&sequence);
    println!("{lis_len} {lis_cnt}", lis_len=result.0, lis_cnt=result.1); 
}