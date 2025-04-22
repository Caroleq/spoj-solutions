/// Solution to Gopu and Validity of Arrangement problem (https://www.spoj.com/problems/SPCU/)

fn read_line_to_i64() -> i64 {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}

fn read_line_to_i64_vec() -> Vec<i64> {
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

fn create_segment_tree(elements_cnt: usize) -> Vec<i64> {

    let segment_tree_len = (get_min_power_geq(elements_cnt) << 1) + 1;
    let mut segment_tree = vec![1; segment_tree_len];

    for idx in (1..(segment_tree_len >> 1)).rev() {
        segment_tree[idx] = segment_tree[2 * idx] + segment_tree[2 * idx + 1];
    }

    return segment_tree;
}

fn find_and_remove_kth(segment_tree: &mut Vec<i64>, k: i64) -> bool {
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

    if segment_tree[idx] == 0 || k != 1{
        return false;
    }

    segment_tree[idx] = 0;
    idx /= 2;
    while idx > 0 {
        segment_tree[idx] = segment_tree[2 * idx] + segment_tree[2 * idx + 1];
        idx /= 2;
    }

    return true;
}

fn compute_ordering(shorter_than_vec: &Vec<i64>) -> bool {
    let mut segment_tree = create_segment_tree(shorter_than_vec.len());

    let mut elements_cnt = shorter_than_vec.len() as i64;
    for shorter_than in shorter_than_vec.iter().rev() {
        let k_th = elements_cnt - *shorter_than;
        if k_th <= 0 || k_th > elements_cnt {
            return false;
        }
        if !find_and_remove_kth(&mut segment_tree, k_th) {
            return false;
        }
        elements_cnt -= 1;
    }

    return true; 
}

fn main() {

    let test_case_cnt = read_line_to_i64();
    for _ in 0..test_case_cnt {
        let _ = read_line_to_i64();
        let shorter_than = read_line_to_i64_vec();
        if shorter_than.len() == 1 {
            if shorter_than[0] == 0 {
                println!("YES");
            }
            else {
                println!("NO");
            }
            
            continue;
        }
        
        if compute_ordering(&shorter_than) {
            println!("YES");
        }
        else {
            println!("NO");
        }        
    }
}