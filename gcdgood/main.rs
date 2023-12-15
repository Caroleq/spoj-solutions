/// Solution to GCD Goodness Problem (https://www.spoj.com/problems/GCDGOOD/)
/// The solution is getting TLE status! 

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

fn gcd(mut n: u64, mut m: u64) -> u64 {
    while m != 0 {
      if m < n {
        std::mem::swap(&mut m, &mut n);
      }
      m %= n;
    }
    return n;
}

fn compute_goodness(array: &mut Vec<u64>) -> u64 {
    array.sort();
    let max_element = array[array.len() - 1] as usize;

    let mut gcd_sums = vec![vec![0; max_element + 1]; 2];
    let mut gcd_counts = vec![vec![0; max_element + 1]; 2];

    gcd_sums[0][array[0] as usize] = array[0];
    gcd_counts[0][array[0] as usize] = 1;

    for last_elem_idx in 1..array.len() {
        let elem_value = array[last_elem_idx];

        let idx = last_elem_idx % 2;
        let prev_idx = (last_elem_idx + 1)% 2;
        gcd_counts[idx] = gcd_counts[prev_idx].to_vec();
        gcd_sums[idx] = gcd_sums[prev_idx].to_vec();

        gcd_counts[idx][elem_value as usize] += 1;
        gcd_sums[idx][elem_value as usize] += elem_value;

        for gcd_val in 1..elem_value as usize + 1 {
            let cnt = gcd_counts[prev_idx][gcd_val];

            let last_elem_and_subseq_gcd = gcd(gcd_val as u64, elem_value) as usize;
            gcd_counts[idx][last_elem_and_subseq_gcd] += cnt;
            gcd_counts[idx][last_elem_and_subseq_gcd] = gcd_counts[idx][last_elem_and_subseq_gcd] % 1000000007;
            gcd_sums[idx][last_elem_and_subseq_gcd] += gcd_sums[prev_idx][gcd_val] + elem_value * cnt;
            gcd_sums[idx][last_elem_and_subseq_gcd] = gcd_sums[idx][last_elem_and_subseq_gcd] % 1000000007;
        }
    }

    let mut total_sum = 0; 

    let result_array = &gcd_sums[(array.len() - 1) % 2];
    for gcd_val in 1..max_element + 1 {
        total_sum += result_array[gcd_val] * (gcd_val as u64);
        total_sum = total_sum % 1000000007;
    }        
    
    return total_sum;
}

fn main() {
    let _ = read_line_to_usize();
    let mut array = read_line_to_u64_vec();
    let goodness = compute_goodness(&mut array);
    println!("{goodness}", goodness=goodness);
}