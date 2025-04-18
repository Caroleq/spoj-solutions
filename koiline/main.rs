/// Solution to Line up problem (https://www.spoj.com/problems/KOILINE/)

fn read_line_to_u64() -> u64 {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
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

fn create_segment_tree(elements_cnt: usize) -> Vec<u64> {

    let segment_tree_len = (get_min_power_geq(elements_cnt) << 1) + 1;
    let mut segment_tree = vec![1; segment_tree_len];

    for idx in (1..(segment_tree_len >> 1)).rev() {
        segment_tree[idx] = segment_tree[2 * idx] + segment_tree[2 * idx + 1];
    }

    return segment_tree;
}

fn find_and_remove_kth(segment_tree: &mut Vec<u64>, k: u64) -> usize {
    let mut k = k;

    let mut idx = 1;
    loop {
        if (2 * idx) < segment_tree.len() && segment_tree[2 * idx] < k {
            k -= segment_tree[2 * idx];
            idx = idx * 2 + 1;
        }
        else {
            idx = idx * 2;
        }

        if idx >= segment_tree.len() / 2 {
            break;
        }
    }

    let returned_idx = idx - segment_tree.len() / 2;
    segment_tree[idx] = 0;
    idx /= 2;
    while idx > 0 {
        segment_tree[idx] = segment_tree[2 * idx] + segment_tree[2 * idx + 1];
        idx /= 2;
    }

    return returned_idx;
}

fn compute_ordering(heights: &Vec<u64>, shorter_than_vec: &Vec<u64>) -> Vec<u64> {
    let mut heights = heights.to_vec();
    heights.sort();
    let mut segment_tree = create_segment_tree(heights.len());
    let mut heights_ordering = Vec::new();

    for shorter_than in shorter_than_vec.iter().rev() {
        let height_idx = find_and_remove_kth(&mut segment_tree, *shorter_than + 1);
        heights_ordering.push(heights[height_idx]);
    }

    heights_ordering.reverse();
    return heights_ordering;
}

fn main() {

    let heights_cnt = read_line_to_u64();
    let mut heights = Vec::new();
    for _ in 0..heights_cnt {
        heights.push(read_line_to_u64());
    }

    let shorter_than = read_line_to_u64_vec();

    let heights_ordering = compute_ordering(&heights, &shorter_than);
    for height in heights_ordering {
        println!("{height}", height=height);
    }
}