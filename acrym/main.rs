// Solution to Acronym Problem (https://www.spoj.com/problems/ACRYM/)

use std::collections::HashSet;

fn read_line_to_usize() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}

fn read_line() {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
}

fn read_lines_to_u8_vecs(lines_number: usize) -> Vec<(String, Vec<u8>)> {
    let mut vector = Vec::new();
    
    for _ in 0..lines_number {
        let mut string = String::new();
        let _ = std::io::stdin().read_line(&mut string);

        vector.push((string.trim().to_string(), string.trim().bytes().collect()));
    }
    return vector;
}

fn compute_acronyms_cnt(words: &Vec<(String, Vec<u8>)>, acronym: &Vec<u8>, optional_words: &HashSet<String>) -> usize {

    let mut dp = vec![vec![0; acronym.len()]; words.len()];

    let mut geatest_no_zero = 0;
    for acr_idx in 0..std::cmp::min(acronym.len(), words[0].1.len()) {
        if words[0].1[0..acr_idx + 1] != acronym[0..acr_idx + 1]{
            break;
        }
        geatest_no_zero = acr_idx;
        dp[0][acr_idx] = 1;
    }

    let mut smallest_no_zero = 0;
    for word_idx in 1..words.len() {

        let is_optional = optional_words.contains(&words[word_idx].0);
        
        let mut new_smallest = std::usize::MAX;
        let mut new_greatest = std::usize::MIN;
        for acr_idx in smallest_no_zero as usize..std::cmp::min(acronym.len(), geatest_no_zero + 50 + 1) {

            for inner_acr_idx in 0..std::cmp::min(acr_idx, words[word_idx].1.len()) {
                if dp[word_idx - 1][acr_idx - inner_acr_idx - 1] > 0 && words[word_idx].1[0..inner_acr_idx + 1] == acronym[acr_idx - inner_acr_idx..acr_idx + 1] {
                    dp[word_idx][acr_idx] += dp[word_idx - 1][acr_idx - inner_acr_idx - 1];
                }
            }

            if is_optional {
                dp[word_idx][acr_idx] += dp[word_idx - 1][acr_idx];
            }

            if dp[word_idx][acr_idx] > 0 {
                new_greatest = acr_idx;
                if new_smallest == std::usize::MAX {
                    new_smallest = acr_idx;
                }
            }
        }
        smallest_no_zero = new_smallest;
        geatest_no_zero = new_greatest;
    }

    return dp[words.len() - 1][acronym.len() - 1];
}

fn main() {
    let tests_count = read_line_to_usize();

    let mut optional_words = HashSet::new();
    optional_words.insert(String::from("in"));
    optional_words.insert(String::from("on"));
    optional_words.insert(String::from("at"));
    optional_words.insert(String::from("to"));
    optional_words.insert(String::from("of"));
    optional_words.insert(String::from("from"));
    optional_words.insert(String::from("for"));
    optional_words.insert(String::from("with"));
    optional_words.insert(String::from("and"));

    for _ in 0..tests_count {
        let words_cnt = read_line_to_usize();
        let acronym = &read_lines_to_u8_vecs(1)[0].1;
        let words = read_lines_to_u8_vecs(words_cnt);
        read_line();

        let acronyms_cnt = compute_acronyms_cnt(&words, &acronym, &optional_words);
        println!("{acronyms_cnt}", acronyms_cnt=acronyms_cnt);
    }
}
