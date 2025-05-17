/// Solution to Number of Palindromes (https://www.spoj.com/problems/NUMOFPAL/)

fn read_line_to_u8_vec() -> Vec<u8> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().as_bytes().to_vec();
}

fn number_of_palindromes(string: &Vec<u8>) -> usize {

    let mut odd_len_string = Vec::new();
    odd_len_string.push(0);
    for idx in 0..string.len() {
        if idx != 0 {odd_len_string.push(0)};
        odd_len_string.push(string[idx]);
    }
    odd_len_string.push(0);

    let mut palindrome_cnt = 0;

    let mut left = 0;
    let mut right = 1;

    let mut p_lens = vec![0 as usize; odd_len_string.len()];

    for idx in 1..p_lens.len() - 1 {
        let mirror_idx = (left as i64 + (right as i64 - idx as i64)) as usize;
        p_lens[idx] = std::cmp::max(0, std::cmp::min(right as i64 - (idx as i64), p_lens[mirror_idx] as i64)) as usize;

        while idx >= p_lens[idx] && idx + p_lens[idx] < odd_len_string.len() && odd_len_string[idx - p_lens[idx]] == odd_len_string[idx + p_lens[idx]] {
            p_lens[idx] += 1;
        }

        if idx + p_lens[idx] > right {
            if idx < p_lens[idx] {
                left = 0;
                right = 2 * idx;
            }
            else {
                left = idx - p_lens[idx];
                right = idx + p_lens[idx];
            }
        }

        if (idx % 2) == 1 {
            palindrome_cnt += (p_lens[idx]) / 2;
        }
        else {
            palindrome_cnt += (p_lens[idx] - 1) / 2;
        }
    }

    return palindrome_cnt;
}

fn main() {

    let case_string = read_line_to_u8_vec();
    let palindromes_cnt = number_of_palindromes(&case_string);
    println!("{palindromes_cnt}", palindromes_cnt=palindromes_cnt);
}