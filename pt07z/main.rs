///  Solution to PT07Z Problem (https://www.spoj.com/problems/PT07Z/)

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

fn parse_tree_from_input() -> Vec<Vec<usize>> {
    let n = read_line_to_usize();
    let mut tree = vec![Vec::<usize>::new(); n];

    for _ in 0..(n - 1) {
        let edge = read_line_to_usize_vec();
        let left = edge[0] - 1;
        let right = edge[1] - 1;

        tree[left].push(right);
        tree[right].push(left);
    }

    return tree;
}

fn dfs(node: usize, parent: usize, tree: &Vec<Vec<usize>>) -> (usize, usize) {
    let mut max_child_height = 0;
    let mut second_child_max_height = 0;

    let mut longest_path = 0;

    for &neighbor in &tree[node] {
        if neighbor == parent {
            continue;
        }

        let (child_height, longest_neighbor_path) = dfs(neighbor, node, &tree);

        if child_height > max_child_height {
            second_child_max_height = max_child_height;
            max_child_height = child_height;
        } else if child_height > second_child_max_height {
            second_child_max_height = child_height;
        }

        longest_path = std::cmp::max(longest_path, longest_neighbor_path);
    }

    let height = std::cmp::max(max_child_height, second_child_max_height) + 1;
    longest_path = std::cmp::max(longest_path, max_child_height + second_child_max_height);
    return (height, longest_path);
}

fn main() {
    let tree = parse_tree_from_input();
    let (_, max_path) = dfs(0, 0, &tree);

    println!("{max_path}", max_path=max_path);
}