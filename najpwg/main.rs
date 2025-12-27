/// Solution to Playing with GCD (https://www.spoj.com/problems/NAJPWG/)

fn read_line_to_usize() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an usize integer");
}

// Implementation based on totient computing from https://www.geeksforgeeks.org/dsa/summation-gcd-pairs-n/ 
fn compute_totient(max_totient: u64) -> Vec<u64> {

    let mut phi = vec![0; max_totient as usize + 1];
    phi[1] = 1;

    for idx_1 in 2..max_totient as usize + 1 {

        if phi[idx_1] == 0 {
            phi[idx_1] = idx_1 as u64 - 1;

            let mut idx_2 = idx_1 << 1;
            while idx_2 as u64 <= max_totient {

                if phi[idx_2] == 0 {
                    phi[idx_2] = idx_2 as u64;
                }
                phi[idx_2] = (phi[idx_2] / idx_1 as u64) * (idx_1 as u64 - 1);
                idx_2 += idx_1;
            }
        }
    }

    return phi;
}

fn compute_pairs(totients: &Vec<u64>) -> Vec<u64> {
    let mut dp = vec![0; totients.len()];
    
    for idx in 2..totients.len() {
        dp[idx] = dp[idx - 1] + (idx as u64) - totients[idx];
    } 
    return dp;
}

fn main() {
    let totients = compute_totient(100000);
    let pairs = compute_pairs(&totients);

    let test_case_cnt = read_line_to_usize();
    for case_id in 1..test_case_cnt + 1 {
        let test_case = read_line_to_usize();
        println!("Case {case_id}: {pairs_cnt}", case_id=case_id, pairs_cnt=pairs[test_case]);
    }
}