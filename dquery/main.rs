/// Solution to D-query Problem (https://www.spoj.com/problems/DQUERY/)
/// Inspired by: https://cp-algorithms.com/data_structures/sqrt_decomposition.html#mos-algorithm

// Testing script usage:
// $ python test_generator.py
// $ ./main.exe < small.input.in > mine.out
// $ diff -w mine.out small.output.out

use std::collections::HashMap;

fn read_line_to_usize() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}

fn read_line_to_u64_vec() -> Vec<u32> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().split(' ').map(|s| s.parse().unwrap()).collect();
}

fn read_lines_to_usize_vecs(lines_number: usize) -> Vec<Vec<usize>> {
    let mut vector = Vec::new();
    
    for _ in 0..lines_number {
        let mut string = String::new();
        let _ = std::io::stdin().read_line(&mut string);
        vector.push(string.trim().split(' ').map(|s| s.parse().unwrap()).collect());
    }

    return vector;
}

fn compute_unique_elements_for_one_block(sequence: &Vec<u32>, queries: &[Vec<usize>], results: &mut Vec<u32>) {
    let mut elements = HashMap::new();

    let mut left_idx = queries[0][0];
    let mut right_idx = queries[0][1];
    for idx in left_idx..right_idx + 1 {
        *elements.entry(sequence[idx]).or_insert(0) += 1;
    }

    for query in queries.iter() {

        while left_idx > query[0] {
            left_idx -= 1;
            *elements.entry(sequence[left_idx]).or_insert(0) += 1;
        }

        while right_idx < query[1] {
            right_idx += 1;
            *elements.entry(sequence[right_idx]).or_insert(0) += 1;
        }

        while left_idx < query[0] {
            if elements[&(sequence[left_idx])] == 1 {
                elements.remove(&(sequence[left_idx]));
            }
            else {
                *elements.get_mut(&sequence[left_idx]).unwrap() -= 1;
            }
            left_idx += 1;
        }

        while right_idx > query[1] {
            if elements[&(sequence[right_idx])] == 1 {
                elements.remove(&(sequence[right_idx]));
            }
            else {
                *elements.get_mut(&sequence[right_idx]).unwrap() -= 1;
            }
            right_idx -= 1;
        }
        results[query[2]] = elements.len() as u32;
    }
}

fn find_block_range(queries: &Vec<Vec<usize>>, block_start_idx: usize) -> usize {
    let mut block_after_end_idx = block_start_idx;

    while block_after_end_idx < queries.len() && queries[block_start_idx][3] == queries[block_after_end_idx][3] {
        block_after_end_idx += 1;
    }
    return block_after_end_idx;
}

fn compute_unique_elements(sequence: &Vec<u32>, queries: &mut Vec<Vec<usize>>) -> Vec<u32> {
    let mut results = vec![0; queries.len()];
    
    queries.sort_by(|q1, q2| {
        let q1_left_block = q1[3] as u32;
        let q2_left_block = q2[3] as u32;

        if q1_left_block < q2_left_block {
            return std::cmp::Ordering::Less;
        }
        else if q1_left_block > q2_left_block {
            return std::cmp::Ordering::Greater;
        }

        if q1[1] > q2[1] {
            return std::cmp::Ordering::Less;
        }
        return std::cmp::Ordering::Greater;
    });

    let mut block_start_idx = 0;
    let mut block_after_end_idx = 0;
    while block_after_end_idx < queries.len() {
        block_after_end_idx = find_block_range(queries, block_start_idx);
        compute_unique_elements_for_one_block(sequence, &queries[block_start_idx..block_after_end_idx], &mut results);
        block_start_idx = block_after_end_idx;
    }

    return results;
}

fn prepare_queries(queries: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let block_size = (queries.len() as f64).sqrt() as u32 / 2;
    return queries.iter().enumerate().map(|(idx, x)| vec![x[0] - 1, x[1] - 1, idx, (x[0] - 1) / block_size as usize]).collect();
}

fn main() {
    let _ = read_line_to_usize();
    let sequence = read_line_to_u64_vec();

    let queries_cnt = read_line_to_usize();
    let queries = read_lines_to_usize_vecs(queries_cnt);
    let mut queries = prepare_queries(&queries);

    let results = compute_unique_elements(&sequence, &mut queries);
    for result in results {
        println!("{result}", result=result);
    }
}