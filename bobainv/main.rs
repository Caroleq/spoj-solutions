/// Solution to Boba Inversion (https://www.spoj.com/problems/BOBAINV/)
/// The solution was getting TLE.

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

fn read_line_to_usize_vec() -> Vec<usize> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().split(' ').map(|s| s.parse().unwrap()).collect();
}

fn precompute_uniqueness_for_all_pairs(sweetness: &Vec<u64>) -> Vec<Vec<u64>> {
    let mut dp = vec![vec![0; sweetness.len() + 1]; sweetness.len() + 1];

    for range_len in 1..sweetness.len() + 1 {
        for left_idx in 0..sweetness.len() - range_len {
            dp[left_idx][left_idx + range_len] = dp[left_idx][left_idx + range_len - 1] + 
                                                 dp[left_idx + 1][left_idx + range_len] -
                                                 dp[left_idx + 1][left_idx + range_len - 1];
            
            if sweetness[left_idx] > sweetness[left_idx + range_len] {
                dp[left_idx][left_idx + range_len] += 1;
            }
        }
    }

    return dp;
}


fn main() {
    let _tea_sequence_cnt = read_line_to_usize();
    let teas_sweetness = read_line_to_u64_vec();

    let uniqueness = precompute_uniqueness_for_all_pairs(&teas_sweetness);

    let queries_cnt = read_line_to_usize();
    for _ in 0..queries_cnt {
        let query = read_line_to_usize_vec();
        let result = uniqueness[query[0] - 1][query[1] - 1];
        println!("{result}", result=result);
    }
}