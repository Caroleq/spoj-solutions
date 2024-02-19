// Solution to Alternating Sequences Problem (https://www.spoj.com/problems/ALTSEQ/)


fn read_line_to_usize() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}

fn read_line_to_i64_vec() -> Vec<i64> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().split(' ').map(|s| s.parse().unwrap()).collect();
}

fn have_different_sign(num_1: i64, num_2: i64) -> bool {
    if num_1 < 0 && num_2 >= 0 {
        return true;
    } else if num_1 >= 0 && num_2 < 0 {
        return true;
    }
    return false;
}

fn main() {
    let seq_size = read_line_to_usize();
    let sequence = read_line_to_i64_vec();

    let mut dp = vec![0; seq_size];

    dp[0] = 1;
    let mut result = 1;
    for i_idx in 1..seq_size {
        dp[i_idx] = 1;
        for j_idx in 0..i_idx {
            if sequence[j_idx].abs() < sequence[i_idx].abs() && have_different_sign(sequence[j_idx], sequence[i_idx])  {
                dp[i_idx] = std::cmp::max(dp[i_idx], dp[j_idx] + 1);
                result = std::cmp::max(result, dp[i_idx]);
            }
        }
    }
    println!("{result}", result=result);
}