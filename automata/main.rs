/// Solution to AUTOMATA - GAME2 problem (https://www.spoj.com/problems/AUTOMATA/)


fn read_line_to_u64() -> u64 {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}

fn read_line_to_u8_vec() -> Vec<u8> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.as_bytes().to_vec();
}

fn if_text_matches_pattern(pattern: &Vec<u8>, text: &Vec<u8>) -> bool {
    let text_len = text.len();
    let pattern_len = pattern.len();

    let wildcard_ascii_code = 42;
    let question_ascii_code = 63;

    let mut dp = vec![vec![false; pattern_len + 1]; text_len + 1];

    dp[0][0] = true;
    for p_idx in 0..pattern_len {
        if pattern[p_idx] == wildcard_ascii_code || pattern[p_idx] == question_ascii_code {
            dp[0][p_idx + 1] = true;
        }
        else {
            break;
        }
    }

    for t_idx in 0..text_len {
        for p_idx in 0..pattern_len {
            
            if pattern[p_idx] == wildcard_ascii_code {
                dp[t_idx + 1][p_idx + 1] = dp[t_idx][p_idx + 1] || dp[t_idx + 1][p_idx];
            }
            else if pattern[p_idx] == question_ascii_code {
                dp[t_idx + 1][p_idx + 1] = dp[t_idx][p_idx] || dp[t_idx + 1][p_idx];
            }
            else if pattern[p_idx] == text[t_idx] {
                dp[t_idx + 1][p_idx + 1] = dp[t_idx][p_idx];
            }
            else {
                dp[t_idx + 1][p_idx + 1] = false; 
            }
        }
    }
    return dp[text_len][pattern_len];
}

fn main() {

    let cases_cnt = read_line_to_u64();
    for _ in 0..cases_cnt {
        let pattern = read_line_to_u8_vec();
        let text = read_line_to_u8_vec();

        let is_match = if_text_matches_pattern(&pattern, &text);
        if is_match {
            println!("Yes");
        }
        else {
            println!("No");
        }
    }
}