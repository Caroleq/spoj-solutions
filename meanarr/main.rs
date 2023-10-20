/// Solution to MEANARR (Mean of array) problem (https://www.spoj.com/problems/MEANARR/)

use std::collections::HashSet;

fn read_line_to_i64_vec() -> Vec<i64> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().split(' ').map(|s| s.parse().unwrap()).collect();
}

fn get_min_power_geq(number: usize) -> usize {
    let mut power_of_two = 1;
    while power_of_two < number {
        power_of_two = power_of_two * 2;
    }
    return power_of_two;
}

#[derive(Clone)]
#[derive(Debug)]
struct Node {
    pub left_range: i64,
    pub mid_range: i64,
    pub right_range: i64,
    pub count: u64
}

fn create_segment_tree(prefix_sums: &Vec<i64>) -> Vec<Node> {

    let mut prefix_sums_set: HashSet<i64> = HashSet::new();
    for number in prefix_sums.iter() {
        prefix_sums_set.insert(*number);
    }

    let segment_tree_len = 2 * get_min_power_geq(prefix_sums_set.len()) + 1;
    let mut segment_tree = vec![Node{left_range: i64::MAX, right_range: i64::MAX, mid_range: i64::MAX, count: 0} ; segment_tree_len];

    let mut idx = segment_tree_len/2;
    let mut sorted_keys = prefix_sums_set.into_iter().collect::<Vec<i64>>();
    sorted_keys.sort();
    for key in sorted_keys {
        segment_tree[idx].left_range = key;
        segment_tree[idx].right_range = key;
        segment_tree[idx].mid_range = key;
        
        idx += 1;
    }

    for idx in (1..(segment_tree_len/2)).rev() {
        segment_tree[idx].left_range = segment_tree[2 * idx].left_range;
        segment_tree[idx].mid_range = segment_tree[2 * idx].right_range;
        segment_tree[idx].right_range = segment_tree[2 * idx + 1].right_range;
    }

    return segment_tree;
}

fn count_subsums_leq(segment_tree: &Vec<Node>, number: i64) -> u64 {
    let mut idx = 1;
    let mut count = 0;
     while segment_tree[idx].left_range != segment_tree[idx].right_range {

         if number > segment_tree[idx].mid_range {
             count += segment_tree[2 * idx].count;
             idx = 2 * idx + 1;
         }
         else {
             idx = 2 * idx;
         }
     }
     count += segment_tree[idx].count;
     return count;
}

fn insert_into_segment_tree(segment_tree: &mut Vec<Node>, number: i64) {
    let mut idx = 1;
    while segment_tree[idx].left_range != segment_tree[idx].right_range {
        segment_tree[idx].count += 1;
        if number > segment_tree[idx].mid_range {
            idx = 2 * idx + 1;
        }
        else {
            idx = 2 * idx;
        }
    }
    segment_tree[idx].count += 1;
}

fn main() {
    let mean = read_line_to_i64_vec()[1];
    let numbers = read_line_to_i64_vec();
    let prefix_sums = numbers.iter().scan(0, |sum, i| {*sum += i -mean; Some(*sum)}).collect::<Vec<_>>();

    let mut segment_tree = create_segment_tree(&prefix_sums);
    
    let mut count: u64 = 0;
    for number in prefix_sums.iter() {

        count += count_subsums_leq(&segment_tree, *number);
        if *number >= 0 {
            count += 1;
        }

        insert_into_segment_tree(&mut segment_tree, *number);
    }
    println!("{count}", count=count);
}