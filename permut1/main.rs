/// Solution to Permutations Problem (https://www.spoj.com/problems/PERMUT1/)

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

 fn compute_permutations(permutation_size: usize, inversions_cnt: usize) -> u32 {

    let mut dp = vec![vec![0; inversions_cnt + 1]; permutation_size + 1];

    for idx in 1..permutation_size + 1 {
        dp[idx][0] = 1;
    }

    for permutation_idx in 1..permutation_size + 1 {
        for inversion_idx in 1..inversions_cnt + 1 {
            let combinations_cnt = permutation_idx * (permutation_idx - 1) / 2;
            if combinations_cnt >= inversion_idx {

                let mut inversions_sum = 0;
                let min_recursive_inv_idx = std::cmp::max(0, (inversion_idx as i32) - (permutation_idx as i32) + 1) as usize;

                for recursive_inv_idx in min_recursive_inv_idx..inversion_idx + 1 {
                    inversions_sum += dp[permutation_idx - 1][recursive_inv_idx];
                }

                dp[permutation_idx][inversion_idx] = inversions_sum;
            }
        }
    }

    return dp[permutation_size][inversions_cnt];
 }

fn main() {
    let cases_count = read_line_to_usize();
    for _ in 0..cases_count {
        let test_case = read_line_to_usize_vec();
        let inversions_cnt = compute_permutations(test_case[0], test_case[1]);
        println!("{inversions_cnt}", inversions_cnt=inversions_cnt);
    }
}