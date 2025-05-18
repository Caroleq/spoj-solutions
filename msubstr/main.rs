/// Solution to Mirror Strings !!! (https://www.spoj.com/problems/MSUBSTR/)

fn read_line_to_usize() -> usize {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().parse().expect("Input not an unsigned integer");
}

fn read_line_to_u8_vec() -> Vec<u8> {
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string.trim().as_bytes().to_vec();
}

fn number_of_palindromes(string: &Vec<u8>) -> (usize, usize) {

    let mut odd_len_string = Vec::new();
    odd_len_string.push(0);
    for idx in 0..string.len() {
        if idx != 0 {odd_len_string.push(0)};
        odd_len_string.push(string[idx]);
    }
    odd_len_string.push(0);

    let mut left = 0;
    let mut right = 1;

    let mut p_lens = vec![0 as usize; odd_len_string.len()];

    let mut max_palindrome_len = 0;
    let mut max_length_palindrome_cnt = 0;

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

        if p_lens[idx] > max_palindrome_len + 1 {
            max_palindrome_len = p_lens[idx] - 1;
            max_length_palindrome_cnt = 1;
        }
        else if p_lens[idx] == max_palindrome_len + 1 {
            max_length_palindrome_cnt += 1;
        }

    }

    return (max_palindrome_len, max_length_palindrome_cnt);
}

fn main() {

    let test_case_cnt = read_line_to_usize();
    for _ in 0..test_case_cnt {

        let case_string = read_line_to_u8_vec();
        let (palindrome_len, palindrome_cnt) = number_of_palindromes(&case_string);
        println!("{palindrome_len} {palindrome_cnt}", palindrome_len=palindrome_len, palindrome_cnt=palindrome_cnt);
    }
}