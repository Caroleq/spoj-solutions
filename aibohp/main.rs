/// Solution to Aibohphobia (AIBOHP) problem: https://www.spoj.com/problems/AIBOHP/

fn read_line_to_usize() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}

fn read_string() -> String {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().to_string();
}

fn compute_lps(string: &str) -> u16 {
    let str_bytes = string.as_bytes();
    let str_len = str_bytes.len();
    let mut dp: Vec<Vec<u16>> = vec![vec![0; str_len + 1]; str_len + 1];

    for i in 1..str_len + 1 {
        let i_idx_char = str_bytes[str_len - i];
        for j in 1..str_len + 1 {

            if i_idx_char == str_bytes[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            }
            else {
                dp[i][j] = std::cmp::max(dp[i][j - 1], dp[i - 1][j]);
            }
        } 
    }

    return dp[str_len][str_len];
}

fn main() {
    let case_count = read_line_to_usize();
    
    for _ in 0..case_count {
        let string = read_string();
        let lps = compute_lps(&string);
        let min_chars_needed = string.len() as u16 - lps;
        println!("{min_chars_needed}", min_chars_needed = min_chars_needed);
    }
}