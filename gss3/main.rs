/// Solution to Can you answer these queries III (https://www.spoj.com/problems/GSS3/)

fn read_line_to_usize() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}

fn read_line_to_i64_vec() -> Vec<i64> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().split(' ').map(|s| s.parse().unwrap()).collect();
}

#[derive(Clone)]
struct Node {
    sum: i64,
    max_prefix_sum: i64,
    max_suffix_sum: i64,
    max_subarray_sum: i64,
}

fn get_2_exp_geq(n: usize) -> usize {
    let mut exp = 1;
    while exp < n {
        exp *= 2;
    }
    return exp;
}

fn get_intermediate_node(left_child: &Node, right_child: &Node) -> Node {
    return Node {
        sum: left_child.sum + right_child.sum,
        max_prefix_sum: std::cmp::max(left_child.max_prefix_sum, left_child.sum + right_child.max_prefix_sum),
        max_suffix_sum: std::cmp::max(right_child.max_suffix_sum, right_child.sum + left_child.max_suffix_sum),
        max_subarray_sum: std::cmp::max(std::cmp::max(left_child.max_subarray_sum, right_child.max_subarray_sum), left_child.max_suffix_sum + right_child.max_prefix_sum),
    };
}

fn create_segment_tree(array: &Vec<i64>) -> Vec<Node> {
    let tree_size = 2 * get_2_exp_geq(array.len()) + 1;
    let mut segment_tree = vec![Node { sum: 0, max_prefix_sum: 0, max_suffix_sum: 0, max_subarray_sum: 0 }; tree_size];

    let start_index = tree_size / 2;
    for i in 0..array.len() {
        segment_tree[start_index + i] = Node {
            sum: array[i],
            max_prefix_sum: array[i],
            max_suffix_sum: array[i],
            max_subarray_sum: array[i],
        };
    }

    for i in (1..start_index).rev() {
        let left_child = &segment_tree[2 * i];
        let right_child = &segment_tree[2 * i + 1];
        
        segment_tree[i] = get_intermediate_node(left_child, right_child);
    }

    return segment_tree;
}

fn update_segment_tree(segment_tree: &mut Vec<Node>, index: usize, value: i64) {
    let mut current_index = segment_tree.len() / 2 + index - 1;
    segment_tree[current_index] = Node {
        sum: value,
        max_prefix_sum: value,
        max_suffix_sum: value,
        max_subarray_sum: value,
    };

    while current_index > 1 {
        current_index /= 2;
        let left_child = &segment_tree[2 * current_index];
        let right_child = &segment_tree[2 * current_index + 1];

        segment_tree[current_index] = get_intermediate_node(left_child, right_child);
    }
}

fn get_maximum_subarray_sum(segment_tree: &Vec<Node>, left: usize, right: usize) -> i64 {
    let mut left_index = segment_tree.len() / 2 + left - 1;
    let mut right_index = segment_tree.len() / 2 + right - 1;

    if left_index == right_index {
        return segment_tree[left_index].sum;
    }

    let mut left_max_suffix_sum = segment_tree[left_index].max_suffix_sum;
    let mut right_max_prefix_sum = segment_tree[right_index].max_prefix_sum;
    let mut max_subarray_sum = std::cmp::max(left_max_suffix_sum, right_max_prefix_sum);

    while left_index / 2 != right_index / 2 {
        if left_index % 2 == 0 {
            let sibling = &segment_tree[left_index + 1];
            max_subarray_sum = std::cmp::max(max_subarray_sum, left_max_suffix_sum + sibling.max_prefix_sum);
            max_subarray_sum = std::cmp::max(max_subarray_sum, sibling.max_subarray_sum);
            left_max_suffix_sum = std::cmp::max(left_max_suffix_sum + sibling.sum, sibling.max_suffix_sum);
        }

        if right_index % 2 == 1 {
            let sibling = &segment_tree[right_index - 1];
            max_subarray_sum = std::cmp::max(max_subarray_sum, sibling.max_suffix_sum + right_max_prefix_sum);
            max_subarray_sum = std::cmp::max(max_subarray_sum, sibling.max_subarray_sum);
            right_max_prefix_sum = std::cmp::max(right_max_prefix_sum + sibling.sum, sibling.max_prefix_sum);
        }

        left_index /= 2;
        right_index /= 2;
    }

    max_subarray_sum = std::cmp::max(max_subarray_sum, left_max_suffix_sum + right_max_prefix_sum);

    return max_subarray_sum;
}

fn main() {
    let problem_size = read_line_to_usize();
    let array = read_line_to_i64_vec();

    let mut segment_tree = create_segment_tree(&array);

    let queries_count = read_line_to_usize();
    for _ in 0..queries_count {
        let query = read_line_to_i64_vec();
        let query_type = query[0];
        if query_type == 0 {
            update_segment_tree(&mut segment_tree, query[1] as usize, query[2]);
        } else {
            let max_subarray_sum = get_maximum_subarray_sum(&segment_tree, query[1] as usize, query[2] as usize);
            println!("{max_sum}", max_sum=max_subarray_sum);
        }
    }
}