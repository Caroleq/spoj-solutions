/// Solution to Euler Totient Function problem (https://www.spoj.com/problems/ETF/)

fn read_line_to_usize() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an usize integer");
}

// Implementation based on totient computing from https://www.geeksforgeeks.org/dsa/summation-gcd-pairs-n/ 
fn compute_totient_old(max_totient: usize) -> Vec<usize> {

    let mut phi = vec![0; max_totient as usize + 1];
    phi[1] = 1;

    for idx_1 in 2..max_totient as usize + 1 {

        if phi[idx_1] == 0 {
            phi[idx_1] = (idx_1 as usize - 1);

            let mut idx_2 = idx_1 << 1;
            while idx_2 <= max_totient as usize {

                if phi[idx_2] == 0 {
                    phi[idx_2] = idx_2 as usize;
                }
                phi[idx_2] = (phi[idx_2] / idx_1) * (idx_1 - 1);
                idx_2 += idx_1;
            }
        }
    }

    return phi;
}

fn main() {
    let test_cases_cnt = read_line_to_usize();
    let totients = compute_totient_old(1000000);
    for _ in 0..test_cases_cnt {
        let number = read_line_to_usize();
        println!("{totient}", totient=totients[number]);
    }
}