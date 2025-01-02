/// Solution to Supernumbers in a permutation (https://www.spoj.com/problems/SUPPER/)
use std::collections::HashSet;
use std::collections::VecDeque;

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

fn insert_into_segment_tree(tree: &mut Vec<(usize,HashSet<usize>)>, position: usize, number: (usize, usize)) {
    let mut idx = position + tree.len() / 2 - 1;
    let max_lis_len = number.0;
    tree[idx].0 = number.0;
    tree[idx].1 = HashSet::from([number.1]);
    
    while idx > 1 {
        idx >>= 1;
        let max_elem = std::cmp::max(tree[2 * idx].0, tree[2 * idx + 1].0);
        if max_elem != max_lis_len {
            return;
        }

        let mut expected_cnt = HashSet::new();
        if tree[2 * idx].0 == max_elem {
            expected_cnt.extend(&tree[2 * idx].1);
        }
        
        if tree[2 * idx + 1].0 == max_elem {
            expected_cnt.extend(&tree[2 * idx + 1].1);
        }

        expected_cnt.insert(position);
        
        tree[idx] = (max_elem, expected_cnt);
    }
}

fn is_left_child(parent: usize, child: usize) -> bool {
    return 2 * parent == child;
}

fn count_max_up_to(segment_tree: &Vec<(usize,HashSet<usize>)>, max_number: usize) -> (usize, HashSet<usize>) {
    if max_number == 0 {
        return (0, HashSet::new());
    }
    let mut idx = max_number + segment_tree.len() / 2 - 1;

    let mut maximum = segment_tree[idx].clone();
    while idx != 1 {
        let last_idx = idx;
        idx >>= 1;
        if is_left_child(idx, last_idx) {
           continue; 
        }

        if segment_tree[2 * idx].0 > maximum.0 {
            maximum = segment_tree[2 * idx].clone();
        }
        else if segment_tree[2 * idx].0 == maximum.0 {
            maximum.1.extend(&segment_tree[2 * idx].1);
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

fn create_segment_tree(sequence_len: usize) -> Vec<(usize,HashSet<usize>)> {
    let tree_len = (get_min_power_geq(sequence_len) << 1) + 1;
    return vec![(0, HashSet::new()); tree_len]; 
}

fn supernumbers_from_predecessors(lis_elems: &HashSet<usize>, predecessors: &Vec<HashSet<usize>>) -> Vec<usize> {
    let mut current_supernumbers = VecDeque::new();
    for number in lis_elems {
        current_supernumbers.push_back(*number);
    }

    let mut is_supernumber = vec![false; predecessors.len() + 1];

    let mut result = Vec::new();
    while !current_supernumbers.is_empty() {
        let number = current_supernumbers.pop_back().unwrap();
        result.push(number);
        is_supernumber[number] = true;
        for elem in &predecessors[number] {
            if !is_supernumber[*elem] {
                is_supernumber[*elem] = true;
                current_supernumbers.push_back(*elem);
            }
        }
    }

    result.sort_unstable();
    return result;
}

fn compute_supernumbers(sequence: &Vec<usize>) -> Vec<usize> {

    if sequence.len() == 1 {
        return vec![sequence[0]];
    }

    let mut predecessors = vec![HashSet::new(); sequence.len() + 1];

    let mut tree = create_segment_tree(sequence.len());
    insert_into_segment_tree(&mut tree, sequence[0], (1, sequence[0]));
    for element in &sequence[1..] {
        let lis = count_max_up_to(&tree, *element - 1);
        predecessors[*element] = lis.1;
        insert_into_segment_tree(&mut tree, *element, (lis.0 + 1, *element));
    }


    return supernumbers_from_predecessors(&tree[1].1, &predecessors);
}

fn main() {
    let test_case_cnt = 10;
    for _ in 0..test_case_cnt {
        let _sequence_size = read_line_to_usize();
        let sequence = read_line_to_usize_vec();

        let supernumbers = compute_supernumbers(&sequence);
        println!("{supernumbers_cnt}", supernumbers_cnt=supernumbers.len());
        for number in supernumbers {
            print!("{number} ", number=number);
        }
        println!(""); 
    }
}