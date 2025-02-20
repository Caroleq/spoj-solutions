/// Solution to Counting binary strings (https://www.spoj.com/problems/STRCOUNT/)

fn compute_function_values() -> Vec<Vec<usize>> {
    let maximum_values_cnt = 63;
    let mut result = vec![vec![0; maximum_values_cnt + 1]; maximum_values_cnt + 1];

    for bin_str_len in 1..maximum_values_cnt + 1 {
        
        for longest_ones_str in 0..bin_str_len + 1 {

            if longest_ones_str == 0 || longest_ones_str == bin_str_len {
                result[bin_str_len][longest_ones_str] = 1;
                continue;
            }

            let mut sum = 0;
            for idx in 0..longest_ones_str {
                sum += result[bin_str_len - idx - 1][longest_ones_str];
            }

            if longest_ones_str + 1 < bin_str_len {
                for idx in 1..longest_ones_str + 1 {
                    sum += result[bin_str_len - longest_ones_str - 1][idx];
                }
            }

            if longest_ones_str + 1 <= bin_str_len {
                sum += 1;
            }

            result[bin_str_len][longest_ones_str] += sum;
        }
    }

    return result;
}

fn main() {

    let result = compute_function_values();
    for str_len_idx in 1..result.len() {
        for longest_ones_idx in 0..str_len_idx + 1 {
            print!("{x} ", x=result[str_len_idx][longest_ones_idx]);
        }
        println!();
    }
}