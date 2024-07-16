/// Solution to IMGPROJ (Image Projections) problem (https://www.spoj.com/problems/IMGPROJ/)
/// 
/// Writeup for the solution: https://caroleq.github.io/Solution-to-IMGPROJ-SPOJ-Problem/
/// Writeup for the solution: https://medium.com/@karolina.gontarek20/imgproj-spoj-segment-trees-in-competative-programming-cabc8bca1950

use std::collections::HashSet;
use std::collections::HashMap;

fn read_line_to_usize() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}

fn read_empty_line() {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
}

fn read_line_to_usize_vec() -> Vec<usize> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().split(' ').map(|s| s.parse().unwrap()).collect();
}

fn read_lines_to_vecs(lines_cnt: usize) -> Vec<Vec<usize>> {
    let mut vecs = Vec::new();
    for _ in 0..lines_cnt {
        vecs.push(read_line_to_usize_vec());
    }
    return vecs;
}

#[derive(Clone)]
#[derive(Debug)]
struct Node {
    pub left_range: usize,
    pub right_range: usize,
    pub sum: u64
}

fn get_min_power_geq(number: usize) -> usize {
    let mut power_of_two = 1;
    while power_of_two < number {
        power_of_two = power_of_two * 2;
    }
    return power_of_two;
}

fn update_left_siblings_node_if_present(segment_tree: &mut Vec<Node>, idx: usize, value: u64) {
    let parent_idx = idx / 2;
    let left_child_idx = 2 * parent_idx;
    if left_child_idx == idx {
        return;
    }
    segment_tree[left_child_idx].sum += value;
}

fn update_right_siblings_node_if_present(segment_tree: &mut Vec<Node>, idx: usize, value: u64) {
    let parent_idx = idx / 2;
    let right_child_idx = 2 * parent_idx + 1;
    if right_child_idx == idx {
        return;
    }
    segment_tree[right_child_idx].sum += value;
}

fn insert_into_segment_tree(mut segment_tree: &mut Vec<Node>, left_idx: usize, right_idx: usize, value: u64) {
    let mut left_idx = left_idx;
    let mut right_idx = right_idx;

    if left_idx == right_idx {
        segment_tree[left_idx].sum += value;
        return;
    }

    segment_tree[left_idx].sum += value;
    segment_tree[right_idx].sum += value;

    while left_idx / 2 != right_idx / 2 {

        update_left_siblings_node_if_present(&mut segment_tree, right_idx, value);
        update_right_siblings_node_if_present(&mut segment_tree, left_idx, value);

        left_idx /= 2;
        right_idx /= 2;
    }
}

fn compute_diag_projection_ranges_recursive(segment_tree: &Vec<Node>, idx: usize, sum: u64, projection_ranges: &mut Vec<(u64, usize)>, maximum_projection_idx: usize) {

    if segment_tree[idx].left_range > maximum_projection_idx {
        return;
    }

    let leaf_offset = segment_tree.len() / 2;
    if idx >= leaf_offset {
        projection_ranges[idx - leaf_offset] = (sum + segment_tree[idx].sum, segment_tree[idx].right_range - segment_tree[idx].left_range + 1);
    } 
    else {
        compute_diag_projection_ranges_recursive(segment_tree, 2 * idx, sum + segment_tree[idx].sum, projection_ranges, maximum_projection_idx);
        compute_diag_projection_ranges_recursive(segment_tree, 2 * idx + 1, sum + segment_tree[idx].sum, projection_ranges, maximum_projection_idx);
    }
}

fn compute_projection_values_change_indexes(rle_matrix: &Vec<Vec<usize>>) -> HashSet<usize> {
    let mut indexes = HashSet::new();
    
    for row_idx in 0..rle_matrix.len() {
        let mut col_idx: usize = 0; 

        let mut diagonal_projection_idx = row_idx;
        indexes.insert(diagonal_projection_idx);

        let rle_row = &rle_matrix[row_idx];
        while col_idx < rle_row.len() {

            let values_count = &rle_row[col_idx];
            diagonal_projection_idx += *values_count;
            indexes.insert(diagonal_projection_idx);

            col_idx += 2;
        }
    }

    return indexes;
}

fn create_segment_tree(rle_matrix: &Vec<Vec<usize>>) -> (Vec<Node>, HashMap<usize, usize>) { 

    let indexes = compute_projection_values_change_indexes(rle_matrix);
    let mut sorted_indexes = indexes.into_iter().collect::<Vec<usize>>();
    sorted_indexes.sort();

    let segment_tree_len = 2 * get_min_power_geq(sorted_indexes.len()) + 1;
    let mut segment_tree = vec![Node{left_range: usize::MAX, right_range: usize::MAX, sum: 0} ; segment_tree_len];

    let mut projection_idx_to_tree_idx = HashMap::new();

    let mut idx = segment_tree_len / 2;

    for key_idx in 0..sorted_indexes.len() {
        let key = sorted_indexes[key_idx];
        segment_tree[idx].left_range = key;

        if key_idx != sorted_indexes.len() - 1 {
            segment_tree[idx].right_range = sorted_indexes[key_idx + 1] - 1;
            projection_idx_to_tree_idx.insert(sorted_indexes[key_idx + 1] - 1, idx);
        }

        projection_idx_to_tree_idx.insert(key, idx);
        idx += 1;
    }

    for idx in (1..(segment_tree_len/2)).rev() {
        segment_tree[idx].left_range = segment_tree[2 * idx].left_range;
        segment_tree[idx].right_range = segment_tree[2 * idx + 1].right_range;
    }

    return (segment_tree, projection_idx_to_tree_idx);
}

fn compute_rle_diagonal_projection_from_segment_tree(segment_tree: &Vec<Node>, cols_cnt: usize, rows_cnt: usize) -> Vec<u64> {

    let mut projection_ranges = vec![(0, 0); segment_tree.len() / 2];
    compute_diag_projection_ranges_recursive(&segment_tree, 1, 0, &mut projection_ranges, cols_cnt + rows_cnt);

    let mut previous_sum = projection_ranges[0].0;
    let mut previous_range_size = projection_ranges[0].1;

    let diagonal_projections_cnt = rows_cnt + cols_cnt - 1;
    let mut processed_proejctions_cnt = previous_range_size;

    let mut idx = 1;

    let mut projection = Vec::new();

    while processed_proejctions_cnt < diagonal_projections_cnt {

        let current_sum = projection_ranges[idx].0;
        let current_range_size = projection_ranges[idx].1;

        if current_sum == previous_sum {
            previous_range_size += current_range_size;
        }
        else {
            projection.push(previous_range_size as u64);
            projection.push(previous_sum);

            previous_range_size = current_range_size;
            previous_sum = current_sum;
        }
        processed_proejctions_cnt += current_range_size;
        idx += 1;
    }
    projection.push(previous_range_size as u64);
    projection.push(previous_sum);

    return projection;

}

fn insert_all_sums_into_segment_tree(rle_matrix: &Vec<Vec<usize>>, segment_tree: &mut Vec<Node>, projection_idx_to_tree_idx: &HashMap<usize, usize>) {

    for rle_row_idx in 0..rle_matrix.len() {

        let rle_row = &rle_matrix[rle_row_idx];
        let mut projection_idx = rle_row_idx;

        let mut rle_matrix_col_idx = 0; 
        while rle_matrix_col_idx < rle_row.len() {
            let value = rle_row[rle_matrix_col_idx + 1] as u64;
            let occurrences_cnt = rle_row[rle_matrix_col_idx] as usize;

            let segment_tree_left_idx = projection_idx_to_tree_idx[&projection_idx];
            let segment_tree_right_idx = projection_idx_to_tree_idx[&(projection_idx + occurrences_cnt - 1)];

            insert_into_segment_tree(segment_tree, segment_tree_left_idx, segment_tree_right_idx, value);
            projection_idx += occurrences_cnt;
            rle_matrix_col_idx += 2;
        }
    }
}

fn compute_diagonal_projection(rle_matrix: &Vec<Vec<usize>>, cols_cnt: usize, rows_cnt: usize) -> Vec<u64> {
    let (mut segment_tree, projection_idx_to_tree_idx) = create_segment_tree(rle_matrix);

    insert_all_sums_into_segment_tree(rle_matrix, &mut segment_tree, &projection_idx_to_tree_idx);

    return compute_rle_diagonal_projection_from_segment_tree(&segment_tree, cols_cnt, rows_cnt);
}

fn main() {

    let cases_cnt = read_line_to_usize();

    for _ in 0..cases_cnt {
        read_empty_line();
        let matrix_dim = read_line_to_usize_vec();

        let rows_cnt = matrix_dim[1] as usize;
        let cols_cnt = matrix_dim[0] as usize;

        let rle_matrix = read_lines_to_vecs(rows_cnt);

        let projection = compute_diagonal_projection(&rle_matrix, rows_cnt, cols_cnt);

        for elem in projection {
            print!("{elem} ", elem=elem);
        }
        println!("");

    }
}