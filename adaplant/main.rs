/// Solution to Ada and Plants problem (https://www.spoj.com/problems/ADAPLANT/)

fn read_line_to_usize() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}

fn read_line_to_u64_vec() -> Vec<u64> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().split(' ').map(|s| s.parse().unwrap()).collect();
}

#[derive(Clone)]
#[derive(Debug)]
struct Node {
    pub min_value: u64,
    pub max_value: u64,
}

fn get_min_power_geq(number: usize) -> usize {
    let mut power_of_two = 1;
    while power_of_two < number {
        power_of_two <<= 1;
    }
    return power_of_two;
}

fn create_segment_tree(plant_heights: &Vec<u64>) -> Vec<Node> { 

    let segment_tree_len = (get_min_power_geq(plant_heights.len()) << 1) + 1;
    let mut segment_tree = vec![Node{min_value: 0, max_value: 0} ; segment_tree_len];

    let mut idx = segment_tree_len >> 1;

    for key_idx in 0..plant_heights.len() {
        let key = plant_heights[key_idx];
        segment_tree[idx].min_value = key;
        segment_tree[idx].max_value = key;

        idx += 1;
    }

    for idx in (1..(segment_tree_len >> 1)).rev() {
        segment_tree[idx].min_value = std::cmp::min(segment_tree[idx << 1].min_value, segment_tree[(idx << 1) + 1].min_value);
        segment_tree[idx].max_value = std::cmp::max(segment_tree[idx << 1].max_value, segment_tree[(idx << 1) + 1].max_value);
    }

    return segment_tree;
}

fn find_max_heights_difference(segment_tree: &Vec<Node>, left: usize, right: usize) -> u64 {

    let mut left = (segment_tree.len() >> 1) + left;
    let mut right = (segment_tree.len() >> 1) + right;

    let mut max_value = std::cmp::max(segment_tree[left].max_value, segment_tree[right].max_value);
    let mut min_value = std::cmp::min(segment_tree[left].min_value, segment_tree[right].min_value);
    
    let mut prev_left = left;
    let mut prev_right = right;
    left >>= 1;
    right >>= 1;
    while left != right {
        if prev_left != (left << 1) + 1 {
            max_value = std::cmp::max(segment_tree[(left << 1) + 1].max_value, max_value);
            min_value = std::cmp::min(segment_tree[(left << 1) + 1].min_value, min_value);
        }

        if prev_right != (right << 1) {
            max_value = std::cmp::max(segment_tree[right << 1].max_value, max_value);
            min_value = std::cmp::min(segment_tree[right << 1].min_value, min_value);
        }

        prev_left = left;
        prev_right = right;
        left >>= 1;
        right >>= 1;
    }
    return max_value - min_value;
}

fn compute_max_heights_difference(plants_heights: &Vec<u64>, max_plants_between_count: usize) -> u64 {

    if plants_heights.len() <= max_plants_between_count + 2 {
        return plants_heights.iter().max().unwrap() - plants_heights.iter().min().unwrap();
    }

    let segment_tree = create_segment_tree(plants_heights);

    let mut max_heights_difference = 0;

    for idx in 0..(plants_heights.len() - max_plants_between_count - 1) {
        let difference  = find_max_heights_difference(&segment_tree, idx, idx + max_plants_between_count + 1);
        max_heights_difference = std::cmp::max(max_heights_difference, difference);
    }

    return max_heights_difference;
}

fn main() {

    let test_case_cnt = read_line_to_usize();

    for _ in 0..test_case_cnt {
        let first_line_of_test_case = read_line_to_u64_vec();
        let max_plants_between_count = first_line_of_test_case[1] as usize;

        let plants_heights = read_line_to_u64_vec();

        let result = compute_max_heights_difference(&plants_heights, max_plants_between_count);
        println!("{result}", result=result);
    }
}