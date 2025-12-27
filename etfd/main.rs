/// Solution to Euler Totient Function Depth (https://www.spoj.com/problems/ETFD/)

fn read_line_to_usize() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an usize integer");
}

fn read_line_to_usize_vec() -> Vec<usize> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().split(' ').map(|s| s.parse().unwrap()).collect();
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

fn compute_depths(totients: &Vec<u64>) -> Vec<Vec<u64>> {
    let mut depths = vec![0; totients.len()];

    for idx in 2..totients.len() {
        depths[idx] = depths[totients[idx] as usize] + 1;
    }
 
    let mut split_depths = vec![vec![0; 20]; totients.len()];
    split_depths[0][0] = 1;
    split_depths[1][0] = 2;
    for idx_1 in 2..totients.len() {
        for idx_2 in 0..20 {
            split_depths[idx_1][idx_2] = split_depths[idx_1 - 1][idx_2];
        }
        if depths[idx_1] > 19 {
            continue;
        }
        split_depths[idx_1][depths[idx_1]] += 1;

    }

    return split_depths;
}

fn main() {
    let totients = compute_totient(1000000);
    let depths = compute_depths(&totients);

    let test_case_cnt = read_line_to_usize();
    for _ in 0..test_case_cnt {
        let test_case = read_line_to_usize_vec();
        let left = test_case[0];
        let right = test_case[1];
        let depth = test_case[2];
        let result = depths[right][depth] - depths[left - 1][depth];
        println!("{result}", result=result);
    }
}