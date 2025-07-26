/// Solution to LCM Sum problem (https://www.spoj.com/problems/LCMSUM/)

fn read_line_to_usize() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an usize integer");
}

fn read_all_cases(test_cases_cnt: usize) -> Vec<u64> {
    let mut test_cases = Vec::new();
    for _ in 0..test_cases_cnt {
        test_cases.push(read_line_to_usize() as u64);
    }
    return test_cases;
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

/// Solved based on https://math.stackexchange.com/questions/761670/how-to-find-this-lcm-sum-function-textlcm1-n-textlcm2-n-cdots-t
fn compute_lcm_sums(totients: &Vec<u64>) -> Vec<u64> {

    let mut lcm_sum = vec![0; totients.len()];
    for divisor in 1..totients.len() as u64 {

        let mut divisor_mult = divisor;
        while divisor_mult < totients.len() as u64 {
            lcm_sum[divisor_mult as usize] += divisor * totients[divisor as usize];
            divisor_mult += divisor;
        }
    }

    for number in 1..totients.len() as u64 {
        lcm_sum[number as usize] = number * (lcm_sum[number as usize] + 1) / 2; 
    }

    return lcm_sum;
}

fn main() {
    let test_cases_cnt = read_line_to_usize();


    let test_cases = read_all_cases(test_cases_cnt);
    let maximum_number = test_cases.iter().max().unwrap();

    let totients = compute_totient(*maximum_number);
    let lcm_sums = compute_lcm_sums(&totients);

    let mut result = String::new();

    for number in test_cases {
        result += &lcm_sums[number as usize].to_string();
        result += "\n";
    }
    print!("{result}", result=result);
}