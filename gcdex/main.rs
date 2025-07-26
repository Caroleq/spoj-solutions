/// Solution to GCD Extreme (https://www.spoj.com/problems/GCDEX/)
/// Implementation based on https://www.geeksforgeeks.org/dsa/summation-gcd-pairs-n/

fn read_line_to_usize() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an usize integer");
}

fn compute_sums(totients: &Vec<u64>) -> Vec<u64> {
    let mut result = vec![0; totients.len()];

    for gcd_val in 1..totients.len() as u64 {

        let mut gcd_multiple = 2;

        while gcd_multiple * gcd_val < totients.len() as u64 {
            result[(gcd_multiple * gcd_val) as usize] += gcd_val * totients[gcd_multiple as usize];
            gcd_multiple += 1;
        } 
    }


    for idx in 2..result.len() {
        result[idx] += result[idx - 1];
    }

    return result;
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

fn main() {
    let totients = compute_totient(1000001);
    let sums = compute_sums(&totients);
    let mut result = String::new(); 
    
    loop {

        let test_case = read_line_to_usize();
        if test_case == 0 {
            break;
        }
        result += &sums[test_case].to_string();
        result += "\n";
    }

    println!("{result}",result=result);
}